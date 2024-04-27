use sos24_domain::entity::permission::Permissions;
use sos24_domain::entity::project::{
    ProjectGroupName, ProjectKanaGroupName, ProjectKanaTitle, ProjectTitle,
};
use sos24_domain::entity::user::UserId;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::repository::Repositories;
use sos24_domain::{ensure, entity::project::Project};

use crate::{
    project::{
        dto::{ProjectAttributeDto, ProjectCategoryDto},
        ProjectUseCase, ProjectUseCaseError,
    },
    shared::context::{ContextProvider, OwnedProject},
    ToEntity,
};

#[derive(Debug)]
pub struct CreateProjectCommand {
    pub title: String,
    pub kana_title: String,
    pub group_name: String,
    pub kana_group_name: String,
    pub category: ProjectCategoryDto,
    pub attributes: Vec<ProjectAttributeDto>,
    pub owner_id: String,
}

impl ToEntity for CreateProjectCommand {
    type Entity = Project;
    type Error = ProjectUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        Ok(Project::create(
            ProjectTitle::try_from(self.title).map_err(ProjectUseCaseError::ProjectTitleError)?,
            ProjectKanaTitle::new(self.kana_title),
            ProjectGroupName::try_from(self.group_name)
                .map_err(ProjectUseCaseError::ProjectGroupNameError)?,
            ProjectKanaGroupName::new(self.kana_group_name),
            self.category.into_entity()?,
            self.attributes.into_entity()?,
            UserId::new(self.owner_id),
        ))
    }
}

impl<R: Repositories> ProjectUseCase<R> {
    pub async fn create(
        &self,
        ctx: &impl ContextProvider,
        raw_project: CreateProjectCommand,
    ) -> Result<String, ProjectUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::CREATE_PROJECT));

        if !self
            .project_application_period
            .can_create_project(&actor, ctx.requested_at())
        {
            return Err(ProjectUseCaseError::ApplicationsNotAccepted);
        }

        let project_id = {
            let lock = self.creation_lock.lock().await;

            if let Some(project) = ctx.project(&*self.repositories).await? {
                let project_id = match project {
                    OwnedProject::Owner(project) => project.value.id().clone(),
                    OwnedProject::SubOwner(project) => project.value.id().clone(),
                };
                return Err(ProjectUseCaseError::AlreadyOwnedProject(project_id));
            }

            let project = raw_project.into_entity()?;
            let project_id = project.id().clone();
            self.repositories
                .project_repository()
                .create(project)
                .await?;

            drop(lock);
            project_id
        };

        Ok(project_id.value().to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::entity::user::UserRole;
    use sos24_domain::test::fixture;
    use sos24_domain::test::repository::MockRepositories;

    use crate::project::dto::ProjectCategoryDto;
    use crate::project::interactor::create::CreateProjectCommand;
    use crate::project::{ProjectUseCase, ProjectUseCaseError};
    use crate::shared::context::TestContext;
    use crate::FromEntity;

    #[tokio::test]
    async fn 一般ユーザーは企画を作成できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_sub_owner_id()
            .returning(|_| Ok(None));
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
                CreateProjectCommand {
                    title: fixture::project::title1().value(),
                    kana_title: fixture::project::kana_title1().value(),
                    group_name: fixture::project::group_name1().value(),
                    kana_group_name: fixture::project::kana_group_name1().value(),
                    category: ProjectCategoryDto::from_entity(fixture::project::category1()),
                    attributes: Vec::from_entity(fixture::project::attributes1()),
                    owner_id: fixture::user::id1().value(),
                },
            )
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn 企画責任者の一般ユーザーは企画を作成できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| {
                Ok(Some(fixture::date::with(fixture::project::project1(
                    fixture::user::id1(),
                ))))
            });
        repositories
            .project_repository_mut()
            .expect_find_by_sub_owner_id()
            .returning(|_| Ok(None));
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .create(
                &ctx,
                CreateProjectCommand {
                    title: fixture::project::title1().value(),
                    kana_title: fixture::project::kana_title1().value(),
                    group_name: fixture::project::group_name1().value(),
                    kana_group_name: fixture::project::kana_group_name1().value(),
                    category: ProjectCategoryDto::from_entity(fixture::project::category1()),
                    attributes: Vec::from_entity(fixture::project::attributes1()),
                    owner_id: fixture::user::id1().value(),
                },
            )
            .await;
        assert!(matches!(
            res,
            Err(ProjectUseCaseError::AlreadyOwnedProject(_))
        ));
    }

    #[tokio::test]
    async fn 実委人は応募期間外に企画を作成できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_sub_owner_id()
            .returning(|_| Ok(None));
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::not_applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .create(
                &ctx,
                CreateProjectCommand {
                    title: fixture::project::title1().value(),
                    kana_title: fixture::project::kana_title1().value(),
                    group_name: fixture::project::group_name1().value(),
                    kana_group_name: fixture::project::kana_group_name1().value(),
                    category: ProjectCategoryDto::from_entity(fixture::project::category1()),
                    attributes: Vec::from_entity(fixture::project::attributes1()),
                    owner_id: fixture::user::id1().value(),
                },
            )
            .await;
        assert!(matches!(
            res,
            Err(ProjectUseCaseError::ApplicationsNotAccepted)
        ));
    }

    #[tokio::test]
    async fn 実委人管理者は応募期間外に企画を作成できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .project_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        repositories
            .project_repository_mut()
            .expect_find_by_owner_id()
            .returning(|_| Ok(None));
        repositories
            .project_repository_mut()
            .expect_find_by_sub_owner_id()
            .returning(|_| Ok(None));
        let use_case = ProjectUseCase::new(
            Arc::new(repositories),
            fixture::project_application_period::not_applicable_period(),
        );

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .create(
                &ctx,
                CreateProjectCommand {
                    title: fixture::project::title1().value(),
                    kana_title: fixture::project::kana_title1().value(),
                    group_name: fixture::project::group_name1().value(),
                    kana_group_name: fixture::project::kana_group_name1().value(),
                    category: ProjectCategoryDto::from_entity(fixture::project::category1()),
                    attributes: Vec::from_entity(fixture::project::attributes1()),
                    owner_id: fixture::user::id1().value(),
                },
            )
            .await;
        assert!(res.is_ok());
    }

    // TODO: 副企画責任者の一般ユーザーは企画を作成できない
}
