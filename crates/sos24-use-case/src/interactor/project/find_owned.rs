use std::sync::Arc;

use sos24_domain::ensure;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::repository::Repositories;

use crate::context::{Context, OwnedProject};
use crate::dto::project::ProjectDto;
use crate::dto::FromEntity;
use crate::interactor::project::{ProjectUseCase, ProjectUseCaseError};

impl<R: Repositories> ProjectUseCase<R> {
    pub async fn find_owned(
        &self,
        ctx: &Context,
    ) -> Result<Option<ProjectDto>, ProjectUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        let project = ctx.project(Arc::clone(&self.repositories)).await?;

        let project = match project {
            Some(OwnedProject::Owner(project)) => project,
            Some(OwnedProject::SubOwner(project)) => project,
            None => return Ok(None),
        };

        ensure!(project.value.is_visible_to(&actor));

        let mut project = ProjectDto::from_entity(project);
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
