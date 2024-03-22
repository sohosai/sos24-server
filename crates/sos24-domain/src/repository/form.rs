use mockall::automock;
use thiserror::Error;

use crate::entity::{
    common::date::WithDate,
    form::{Form, FormId},
};

#[derive(Debug, Error)]
pub enum FormRepositoryError {
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait FormRepository: Send + Sync + 'static {
    async fn list(&self) -> Result<Vec<WithDate<Form>>, FormRepositoryError>;
    async fn create(&self, form: Form) -> Result<(), FormRepositoryError>;
    async fn find_by_id(&self, id: FormId) -> Result<Option<WithDate<Form>>, FormRepositoryError>;
    async fn update(&self, form: Form) -> Result<(), FormRepositoryError>;
    async fn delete_by_id(&self, id: FormId) -> Result<(), FormRepositoryError>;
}
