use std::sync::Arc;

use sos24_domain::entity::news::NewsIdError;
use sos24_domain::entity::news_attachment::{NewsAttachmentId, NewsAttachmentIdError};
use sos24_domain::repository::news_attachment::{
    NewsAttachmentRepository, NewsAttachmentRepositoryError,
};
use sos24_domain::{
    ensure,
    entity::{
        actor::Actor,
        permission::{PermissionDeniedError, Permissions},
    },
    repository::Repositories,
};
use thiserror::Error;

use crate::dto::{
    news_attachment::CreateNewsAttachmentDto, news_attachment::NewsAttachmentDto, FromEntity,
    ToEntity,
};

#[derive(Debug, Error)]
pub enum NewsAttachmentUseCaseError {
    #[error("News attachment not found: {0:?}")]
    NotFound(NewsAttachmentId),

    #[error(transparent)]
    NewsAttachmentRepositoryError(#[from] NewsAttachmentRepositoryError),
    #[error(transparent)]
    NewsAttachmentIdError(#[from] NewsAttachmentIdError),
    #[error(transparent)]
    NewsAttachmentNewsIdError(#[from] NewsIdError),
    #[error(transparent)]
    NewsAttachmentUrlError(#[from] url::ParseError),
    #[error(transparent)]
    PermissionDeniedError(#[from] PermissionDeniedError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

pub struct NewsAttachmentUseCase<R: Repositories> {
    repositories: Arc<R>,
}

impl<R: Repositories> NewsAttachmentUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn list(
        &self,
        actor: &Actor,
    ) -> Result<Vec<NewsAttachmentDto>, NewsAttachmentUseCaseError> {
        ensure!(actor.has_permission(Permissions::READ_NEWS_ALL));

        let raw_news_attachment_list = self
            .repositories
            .news_attachment_repository()
            .list()
            .await?;
        let news_attachment_list = raw_news_attachment_list
            .into_iter()
            .map(NewsAttachmentDto::from_entity);
        Ok(news_attachment_list.collect())
    }

    pub async fn create(
        &self,
        actor: &Actor,
        raw_news_attachment: CreateNewsAttachmentDto,
    ) -> Result<(), NewsAttachmentUseCaseError> {
        ensure!(actor.has_permission(Permissions::CREATE_NEWS));

        let news_attachment = raw_news_attachment.into_entity()?;
        self.repositories
            .news_attachment_repository()
            .create(news_attachment)
            .await?;
        Ok(())
    }

    pub async fn find_by_id(
        &self,
        actor: &Actor,
        id: String,
    ) -> Result<NewsAttachmentDto, NewsAttachmentUseCaseError> {
        ensure!(actor.has_permission(Permissions::READ_NEWS_ALL));

        let id = NewsAttachmentId::try_from(id)?;
        let raw_news_attachment = self
            .repositories
            .news_attachment_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(NewsAttachmentUseCaseError::NotFound(id))?;
        Ok(NewsAttachmentDto::from_entity(raw_news_attachment))
    }

    pub async fn delete_by_id(
        &self,
        actor: &Actor,
        id: String,
    ) -> Result<(), NewsAttachmentUseCaseError> {
        ensure!(actor.has_permission(Permissions::DELETE_NEWS_ALL));

        let id = NewsAttachmentId::try_from(id)?;
        self.repositories
            .news_attachment_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(NewsAttachmentUseCaseError::NotFound(id.clone()))?;

        self.repositories
            .news_attachment_repository()
            .delete_by_id(id)
            .await?;
        Ok(())
    }
}
