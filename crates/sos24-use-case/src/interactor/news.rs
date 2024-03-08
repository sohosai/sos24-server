use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{
        actor::Actor,
        news::{NewsBody, NewsCategories, NewsId, NewsIdError, NewsTitle},
        permission::{PermissionDeniedError, Permissions},
    },
    repository::{
        news::{NewsRepository, NewsRepositoryError},
        Repositories,
    },
};
use thiserror::Error;

use crate::dto::FromEntity;
use crate::dto::{
    news::{CreateNewsDto, NewsDto, UpdateNewsDto},
    ToEntity,
};

#[derive(Debug, Error)]
pub enum NewsUseCaseError {
    #[error("News not found: {0:?}")]
    NotFound(NewsId),

    #[error(transparent)]
    NewsRepositoryError(#[from] NewsRepositoryError),
    #[error(transparent)]
    NewsIdError(#[from] NewsIdError),
    #[error(transparent)]
    PermissionDeniedError(#[from] PermissionDeniedError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

pub struct NewsUseCase<R: Repositories> {
    repositories: Arc<R>,
}

impl<R: Repositories> NewsUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn list(&self, actor: &Actor) -> Result<Vec<NewsDto>, NewsUseCaseError> {
        ensure!(actor.has_permission(Permissions::READ_NEWS_ALL));

        let raw_news_list = self.repositories.news_repository().list().await?;
        let news_list = raw_news_list.into_iter().map(NewsDto::from_entity);
        Ok(news_list.collect())
    }

    pub async fn create(
        &self,
        actor: &Actor,
        raw_news: CreateNewsDto,
    ) -> Result<(), NewsUseCaseError> {
        ensure!(actor.has_permission(Permissions::CREATE_NEWS));

        let news = raw_news.into_entity()?;
        self.repositories.news_repository().create(news).await?;
        Ok(())
    }

    pub async fn find_by_id(&self, actor: &Actor, id: String) -> Result<NewsDto, NewsUseCaseError> {
        ensure!(actor.has_permission(Permissions::READ_NEWS_ALL));

        let id = NewsId::try_from(id)?;
        let raw_news = self
            .repositories
            .news_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(NewsUseCaseError::NotFound(id))?;
        Ok(NewsDto::from_entity(raw_news))
    }

    pub async fn update(
        &self,
        actor: &Actor,
        news_data: UpdateNewsDto,
    ) -> Result<(), NewsUseCaseError> {
        let id = NewsId::try_from(news_data.id)?;
        let news = self
            .repositories
            .news_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(NewsUseCaseError::NotFound(id))?;

        let mut new_news = news.value;
        new_news.set_title(actor, NewsTitle::new(news_data.title))?;
        new_news.set_body(actor, NewsBody::new(news_data.body))?;
        new_news.set_categories(actor, NewsCategories::new(news_data.categories))?;

        self.repositories.news_repository().update(new_news).await?;
        Ok(())
    }

    pub async fn delete_by_id(&self, actor: &Actor, id: String) -> Result<(), NewsUseCaseError> {
        ensure!(actor.has_permission(Permissions::DELETE_NEWS_ALL));

        let id = NewsId::try_from(id)?;
        self.repositories
            .news_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(NewsUseCaseError::NotFound(id.clone()))?;

        self.repositories.news_repository().delete_by_id(id).await?;
        Ok(())
    }
}
