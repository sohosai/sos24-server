use mockall::automock;
use thiserror::Error;

use crate::entity::common::date::WithDate;
use crate::entity::form::FormId;
use crate::entity::form_answer::{FormAnswer, FormAnswerId};
use crate::entity::project::ProjectId;

#[derive(Debug, Error)]
pub enum FormAnswerRepositoryError {
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait FormAnswerRepository: Send + Sync + 'static {
    async fn list(&self) -> Result<Vec<WithDate<FormAnswer>>, FormAnswerRepositoryError>;
    async fn create(&self, form_answer: FormAnswer) -> Result<(), FormAnswerRepositoryError>;
    async fn find_by_id(
        &self,
        id: FormAnswerId,
    ) -> Result<Option<WithDate<FormAnswer>>, FormAnswerRepositoryError>;
    async fn find_by_project_id(
        &self,
        project_id: ProjectId,
    ) -> Result<Vec<WithDate<FormAnswer>>, FormAnswerRepositoryError>;
    async fn find_by_form_id(
        &self,
        form_id: FormId,
    ) -> Result<Vec<WithDate<FormAnswer>>, FormAnswerRepositoryError>;
    async fn find_by_project_id_and_form_id(
        &self,
        project_id: ProjectId,
        form_id: FormId,
    ) -> Result<Option<WithDate<FormAnswer>>, FormAnswerRepositoryError>;
    async fn update(&self, form_answer: FormAnswer) -> Result<(), FormAnswerRepositoryError>;
}
