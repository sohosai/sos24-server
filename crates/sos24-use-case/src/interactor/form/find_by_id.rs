use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{form::FormId, permission::Permissions},
    repository::{form::FormRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{form::FormDto, FromEntity},
};

use super::{FormUseCase, FormUseCaseError};

impl<R: Repositories> FormUseCase<R> {
    pub async fn find_by_id(&self, ctx: &Context, id: String) -> Result<FormDto, FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_FORM_ALL));

        let id = FormId::try_from(id)?;
        let form = self
            .repositories
            .form_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(FormUseCaseError::NotFound(id))?;

        Ok(FormDto::from_entity(form))
    }
}

#[cfg(test)]
mod tests {
    // TODO: 一般ユーザーは申請を取得できる
}
