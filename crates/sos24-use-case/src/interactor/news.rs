use std::sync::Arc;

use anyhow::Context;
use sos24_domain::{
    ensure,
    entity::{
        actor::Actor,
        news::{NewsBody, NewsCategories, NewsId, NewsTitle},
        permission::Permissions,
    },
    repository::{news::NewsRepository, Repositories},
};

use crate::{
    dto::FromEntity,
    error::{news::NewsError, UseCaseError},
};
use crate::{
    dto::{
        news::{CreateNewsDto, NewsDto, UpdateNewsDto},
        ToEntity,
    },
    error::Result,
};

pub struct NewsUseCase<R: Repositories> {
    repositories: Arc<R>,
}

impl<R: Repositories> NewsUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn list(&self, actor: &Actor) -> Result<Vec<NewsDto>, NewsError> {
        ensure!(actor.has_permission(Permissions::READ_NEWS_ALL));

        let raw_news_list = self
            .repositories
            .news_repository()
            .list()
            .await
            .context("Failed to list news")?;

        let news_list = raw_news_list
            .into_iter()
            .map(NewsDto::from_entity)
            .collect();
        Ok(news_list)
    }

    pub async fn create(&self, actor: &Actor, raw_news: CreateNewsDto) -> Result<(), NewsError> {
        ensure!(actor.has_permission(Permissions::CREATE_NEWS));

        let news = raw_news.into_entity()?;
        self.repositories
            .news_repository()
            .create(news)
            .await
            .context("Failed to create news")?;
        Ok(())
    }

    pub async fn find_by_id(&self, actor: &Actor, id: String) -> Result<NewsDto, NewsError> {
        ensure!(actor.has_permission(Permissions::READ_NEWS_ALL));

        let id: NewsId = id.try_into()?;
        let raw_news = self
            .repositories
            .news_repository()
            .find_by_id(id.clone())
            .await
            .context("Failed to find news")?;

        match raw_news {
            Some(raw_news) => Ok(NewsDto::from_entity(raw_news)),
            None => Err(UseCaseError::UseCase(NewsError::NotFound(id))),
        }
    }

    pub async fn update(&self, actor: &Actor, news_data: UpdateNewsDto) -> Result<(), NewsError> {
        ensure!(actor.has_permission(Permissions::UPDATE_NEWS_ALL));

        let id: NewsId = news_data.id.try_into().context("Failed to parse news id")?;
        let news = self
            .repositories
            .news_repository()
            .find_by_id(id.clone())
            .await
            .context("Failed to find news")?
            .ok_or(UseCaseError::UseCase(NewsError::NotFound(id)))?;

        let mut new_news = news.value;
        new_news.set_title(actor, NewsTitle::new(news_data.title))?;
        new_news.set_body(actor, NewsBody::new(news_data.body))?;
        new_news.set_categories(actor, NewsCategories::new(news_data.categories))?;

        self.repositories
            .news_repository()
            .update(new_news)
            .await
            .context("Failed to update news")?;

        Ok(())
    }

    pub async fn delete_by_id(&self, actor: &Actor, id: String) -> Result<(), NewsError> {
        ensure!(actor.has_permission(Permissions::DELETE_NEWS_ALL));

        let id: NewsId = id.try_into().context("Failed to parse news id")?;

        self.repositories
            .news_repository()
            .find_by_id(id.clone())
            .await
            .context("Failed to find news")?
            .ok_or(UseCaseError::UseCase(NewsError::NotFound(id.clone())))?;

        self.repositories
            .news_repository()
            .delete_by_id(id)
            .await
            .context("Failed to delete news")?;

        Ok(())
    }
}
