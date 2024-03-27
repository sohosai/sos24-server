use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{form::FormId, permission::Permissions},
    repository::{form::FormRepository, Repositories},
};

use crate::context::Context;

use super::{FormUseCase, FormUseCaseError};

impl<R: Repositories> FormUseCase<R> {
    pub async fn delete_by_id(&self, ctx: &Context, id: String) -> Result<(), FormUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::DELETE_FORM_ALL));

        let id = FormId::try_from(id)?;
        self.repositories
            .form_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(FormUseCaseError::NotFound(id.clone()))?;

        self.repositories.form_repository().delete_by_id(id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // TODO: 実委人は申請を削除できない
    // TODO: 実委人管理者は申請を削除できる
}
