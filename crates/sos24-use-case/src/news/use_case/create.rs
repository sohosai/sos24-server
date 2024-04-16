use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{
        file_data::FileId,
        news::{News, NewsBody, NewsTitle},
        permission::Permissions,
        project::{ProjectAttributes, ProjectCategories},
    },
    repository::{file_data::FileDataRepository, news::NewsRepository, Repositories},
};

use crate::{
    context::Context,
    project::dto::{ProjectAttributesDto, ProjectCategoriesDto},
};

use super::{NewsUseCase, NewsUseCaseError};

pub struct CreateNewsCommand {
    pub title: String,
    pub body: String,
    pub attachments: Vec<String>,
    pub categories: ProjectCategoriesDto,
    pub attributes: ProjectAttributesDto,
}

impl<R: Repositories> NewsUseCase<R> {
    pub async fn create(
        &self,
        ctx: &Context,
        news_data: CreateNewsCommand,
    ) -> Result<String, NewsUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::CREATE_NEWS));

        let news = News::create(
            NewsTitle::new(news_data.title),
            NewsBody::new(news_data.body),
            news_data
                .attachments
                .into_iter()
                .map(FileId::try_from)
                .collect::<Result<_, _>>()?,
            ProjectCategories::from(news_data.categories),
            ProjectAttributes::from(news_data.attributes),
        );

        for file_id in news.attachments() {
            let _ = self
                .repositories
                .file_data_repository()
                .find_by_id(file_id.clone())
                .await?
                .ok_or(NewsUseCaseError::FileNotFound(file_id.clone()))?;
        }

        let news_id = news.id().clone();
        self.repositories.news_repository().create(news).await?;

        Ok(news_id.value().to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::{
        entity::{permission::PermissionDeniedError, user::UserRole},
        test::{fixture, repository::MockRepositories},
    };

    use crate::{
        context::Context,
        news::use_case::{create::CreateNewsCommand, NewsUseCase, NewsUseCaseError},
        project::dto::{ProjectAttributesDto, ProjectCategoriesDto},
    };

    #[tokio::test]
    async fn 実委人はお知らせを作成できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = NewsUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .create(
                &ctx,
                CreateNewsCommand {
                    title: fixture::news::title1().value(),
                    body: fixture::news::body1().value(),
                    attachments: fixture::news::attachments1()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    categories: ProjectCategoriesDto::from(fixture::news::categories1()),
                    attributes: ProjectAttributesDto::from(fixture::news::attributes1()),
                },
            )
            .await;
        assert!(matches!(
            res,
            Err(NewsUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn 実委人管理者はお知らせを作成できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = NewsUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .create(
                &ctx,
                CreateNewsCommand {
                    title: fixture::news::title1().value(),
                    body: fixture::news::body1().value(),
                    attachments: fixture::news::attachments1()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    categories: ProjectCategoriesDto::from(fixture::news::categories1()),
                    attributes: ProjectAttributesDto::from(fixture::news::attributes1()),
                },
            )
            .await;
        assert!(res.is_ok());
    }
}
