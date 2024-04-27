use sos24_domain::ensure;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::repository::user::UserRepository;
use sos24_domain::repository::Repositories;

use crate::{
    context::{ContextProvider, OwnedProject},
    project::{dto::ProjectDto, ProjectUseCase, ProjectUseCaseError},
    FromEntity,
};

impl<R: Repositories> ProjectUseCase<R> {
    pub async fn find_owned(
        &self,
        ctx: &impl ContextProvider,
    ) -> Result<Option<ProjectDto>, ProjectUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        let project = ctx.project(&*self.repositories).await?;

        let raw_project = match project {
            Some(OwnedProject::Owner(project)) => project,
            Some(OwnedProject::SubOwner(project)) => project,
            None => return Ok(None),
        };

        ensure!(raw_project.value.is_visible_to(&actor));

        let owner_id = raw_project.value.owner_id();
        let raw_owner = self
            .repositories
            .user_repository()
            .find_by_id(owner_id.clone())
            .await?
            .ok_or(ProjectUseCaseError::UserNotFound(owner_id.clone()))?;

        let sub_owner_id = raw_project.value.sub_owner_id();
        let raw_sub_owner = match sub_owner_id {
            Some(sub_owner_id) => Some(
                self.repositories
                    .user_repository()
                    .find_by_id(sub_owner_id.clone())
                    .await?
                    .ok_or(ProjectUseCaseError::UserNotFound(sub_owner_id.clone()))?,
            ),
            None => None,
        };

        let mut project = ProjectDto::from_entity((raw_project, raw_owner, raw_sub_owner));
        if !actor.has_permission(Permissions::READ_PROJECT_ALL) {
            project.remarks = None;
        }

        Ok(Some(project))
    }
}

#[cfg(test)]
mod tests {
    // TODO: 一般ユーザーは備考を取得できない
    // TODO: 実委人は備考を取得できる
}
