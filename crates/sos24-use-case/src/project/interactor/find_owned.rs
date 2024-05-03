use sos24_domain::ensure;
use sos24_domain::entity::permission::Permissions;
use sos24_domain::repository::Repositories;

use crate::{
    project::{dto::ProjectDto, ProjectUseCase, ProjectUseCaseError},
    shared::{adapter::Adapters, context::ContextProvider},
};

impl<R: Repositories, A: Adapters> ProjectUseCase<R, A> {
    pub async fn find_owned(
        &self,
        ctx: &impl ContextProvider,
    ) -> Result<Option<ProjectDto>, ProjectUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;

        let Some(project_with_owners) = ctx.project(&*self.repositories).await? else {
            return Ok(None);
        };
        ensure!(project_with_owners.project.is_visible_to(&actor));

        let mut project = ProjectDto::from(project_with_owners);
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
