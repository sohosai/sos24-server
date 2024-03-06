use std::sync::Arc;

use anyhow::Context;
use sos24_domain::{
    entity::news::{News, NewsBody, NewsCategories, NewsId, NewsTitle},
    repository::{news::NewsRepository, Repositories},
};

use crate::error::UseCaseError;
use crate::{
    dto::news::{CreateNewsDto, NewsDto, UpdateNewsDto},
    error::Result,
};

pub struct NewsUseCase<R: Repositories> {
    repositories: Arc<R>,
}

#[derive(Debug)]
pub enum NewsError {
    NotFound,
}

impl<R: Repositories> NewsUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn list(&self) -> Result<Vec<NewsDto>, NewsError> {
        // TODO: 権限チェック
        let raw_news_list = self
            .repositories
            .news_repository()
            .list()
            .await
            .context("Failed to list news")?;

        let news_list = raw_news_list.into_iter().map(NewsDto::from).collect();
        Ok(news_list)
    }

    pub async fn create(&self, raw_news: CreateNewsDto) -> Result<(), NewsError> {
        // TODO: 権限チェック
        let news = News::from(raw_news);
        self.repositories
            .news_repository()
            .create(news)
            .await
            .context("Failed to create news")?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> Result<NewsDto, NewsError> {
        // TODO: 権限チェック
        let id: NewsId = id.try_into().context("Failed to parse news id")?;
        let raw_news = self
            .repositories
            .news_repository()
            .find_by_id(id)
            .await
            .context("Failed to find news")?;

        match raw_news {
            Some(raw_news) => Ok(NewsDto::from(raw_news)),
            None => Err(UseCaseError::UseCase(NewsError::NotFound)),
        }
    }

    pub async fn update(&self, news_data: UpdateNewsDto) -> Result<(), NewsError> {
        // TODO: 権限チェック
        let id: NewsId = news_data.id.try_into().context("Failed to parse news id")?;
        let news = self
            .repositories
            .news_repository()
            .find_by_id(id.clone())
            .await
            .context("Failed to find news")?
            .ok_or(UseCaseError::UseCase(NewsError::NotFound))?;

        let mut new_news = news;
        new_news.title = NewsTitle::new(news_data.title);
        new_news.body = NewsBody::new(news_data.body);
        new_news.categories = NewsCategories::new(news_data.categories);
        new_news.updated_at = chrono::Utc::now();

        self.repositories
            .news_repository()
            .update(new_news)
            .await
            .context("Failed to update news")?;

        Ok(())
    }

    pub async fn delete_by_id(&self, id: &str) -> Result<(), NewsError> {
        // TODO: 権限チェック
        let id: NewsId = id.try_into().context("Failed to parse news id")?;

        self.repositories
            .news_repository()
            .find_by_id(id.clone())
            .await
            .context("Failed to find news")?
            .ok_or(UseCaseError::UseCase(NewsError::NotFound))?;

        self.repositories
            .news_repository()
            .delete_by_id(id)
            .await
            .context("Failed to delete news")?;

        Ok(())
    }
}
