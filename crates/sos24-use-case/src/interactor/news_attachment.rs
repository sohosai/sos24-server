use std::sync::Arc;

use sos24_domain::entity::news::NewsIdError;
use sos24_domain::entity::news_attachment_data::{
    NewsAttachmentData, NewsAttachmentFilename, NewsAttachmentId, NewsAttachmentIdError,
};
use sos24_domain::entity::news_attachment_object::{NewsAttachmentObject, NewsAttachmentObjectKey};
use sos24_domain::repository::news_attachment_data::{
    NewsAttachmentRepository, NewsAttachmentRepositoryError,
};
use sos24_domain::repository::news_attachment_object::{
    NewsAttachmentObjectRepository, NewsAttachmentObjectRepositoryError,
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

use crate::context::Context;
use crate::dto::FromEntity;
use crate::dto::{news_attachment::CreateNewsAttachmentDto, news_attachment::NewsAttachmentDto};

#[derive(Debug, Error)]
pub enum NewsAttachmentUseCaseError {
    #[error("News attachment not found: {0:?}")]
    NotFound(NewsAttachmentId),
    #[error(transparent)]
    NewsAttachmentRepositoryError(#[from] NewsAttachmentRepositoryError),
    #[error(transparent)]
    NewsAttachmentObjectRepositoryError(#[from] NewsAttachmentObjectRepositoryError),
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
        bucket: String,
        actor: &Actor,
    ) -> Result<Vec<NewsAttachmentDto>, NewsAttachmentUseCaseError> {
        // ToDo: 権限
        ensure!(actor.has_permission(Permissions::READ_NEWS_ALL));
        let raw_news_attachment_list = self
            .repositories
            .news_attachment_repository()
            .list()
            .await?;
        let mut news_attachment_list: Vec<NewsAttachmentDto> = vec![];
        for news_attachment_data in raw_news_attachment_list {
            let url = self
                .repositories
                .news_attachment_object_repository()
                .generate_url(bucket.clone(), news_attachment_data.value.url().copy())
                .await?;
            news_attachment_list.push(NewsAttachmentDto::from_entity((
                news_attachment_data,
                url.value().into(),
            )));
        }
        Ok(news_attachment_list)
    }

    pub async fn create(
        &self,
        bucket: String,
        key_prefix: String,
        raw_news_attachment: CreateNewsAttachmentDto,
    ) -> Result<(), NewsAttachmentUseCaseError> {
        // ToDo: 権限・所有者設定
        let key = NewsAttachmentObjectKey::generate(
            key_prefix.as_str(),
            raw_news_attachment.filename.as_str(),
        );

        let object = NewsAttachmentObject::new(raw_news_attachment.file, key.clone());
        self.repositories
            .news_attachment_object_repository()
            .create(bucket, object)
            .await?;

        let data = NewsAttachmentData::create(
            NewsAttachmentFilename::new(raw_news_attachment.filename),
            key,
        );
        self.repositories
            .news_attachment_repository()
            .create(data)
            .await?;
        Ok(())
    }

    pub async fn find_by_id(
        &self,
        ctx: &Context,
        backet: String,
        id: String,
    ) -> Result<NewsAttachmentDto, NewsAttachmentUseCaseError> {
        let id = NewsAttachmentId::try_from(id)?;
        let raw_news_attachment = self
            .repositories
            .news_attachment_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(NewsAttachmentUseCaseError::NotFound(id))?;
        let signed_url = self
            .repositories
            .news_attachment_object_repository()
            .generate_url(backet, raw_news_attachment.value.url().copy())
            .await?
            .value()
            .into();
        Ok(NewsAttachmentDto::from_entity((
            raw_news_attachment,
            signed_url,
        )))
    }

    pub async fn delete_by_id(
        &self,
        actor: &Actor,
        id: String,
    ) -> Result<(), NewsAttachmentUseCaseError> {
        ensure!(actor.has_permission(Permissions::DELETE_NEWS_ALL));

        // ソフトデリートで実装している（オブジェクトストレージからは削除されない）
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
