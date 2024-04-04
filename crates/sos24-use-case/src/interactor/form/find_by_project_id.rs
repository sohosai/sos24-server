use std::sync::Arc;

use sos24_domain::entity::project::ProjectId;
use sos24_domain::repository::form_answer::FormAnswerRepository;
use sos24_domain::repository::project::ProjectRepository;
use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{form::FormRepository, Repositories},
};

use crate::dto::form::FormSummaryDto;
use crate::{context::Context, dto::FromEntity};

use super::{FormUseCase, FormUseCaseError};

impl<R: Repositories> FormUseCase<R> {
    pub async fn find_by_project_id(
        &self,
        ctx: &Context,
        project_id: String,
    ) -> Result<Vec<FormSummaryDto>, FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_FORM_ALL));

        let project_id = ProjectId::try_from(project_id)?;
        let project = self
            .repositories
            .project_repository()
            .find_by_id(project_id.clone())
            .await?
            .ok_or(FormUseCaseError::ProjectNotFound(project_id.clone()))?;
        ensure!(project.value.is_visible_to(&actor));

        let forms = self.repositories.form_repository().list().await?;

        // FIXME : N+1
        let mut form_list = vec![];
        for form in forms {
            let form_id = form.value.id().clone();
            let form_answer = self
                .repositories
                .form_answer_repository()
                .find_by_project_id_and_form_id(project_id.clone(), form_id)
                .await?;
            form_list.push(FormSummaryDto::from_entity((form, form_answer)));
        }
        Ok(form_list)
    }
}

#[cfg(test)]
mod tests {}
