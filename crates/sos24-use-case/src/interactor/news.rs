use std::sync::Arc;

use thiserror::Error;

use sos24_domain::{
    ensure,
    entity::{
        news::{NewsBody, NewsId, NewsIdError, NewsTitle},
        permission::{PermissionDeniedError, Permissions},
    },
    repository::{
        news::{NewsRepository, NewsRepositoryError},
        Repositories,
    },
};

use crate::interactor::project::ProjectUseCaseError;
use crate::{context::Context, dto::FromEntity};
use crate::{
    context::ContextError,
    dto::{
        news::{CreateNewsDto, NewsDto, UpdateNewsDto},
        ToEntity,
    },
};

#[derive(Debug, Error)]
pub enum NewsUseCaseError {
    #[error("News not found: {0:?}")]
    NotFound(NewsId),

    #[error(transparent)]
    ProjectUseCaseError(#[from] ProjectUseCaseError),
    #[error(transparent)]
    ContextError(#[from] ContextError),
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

    pub async fn list(&self, ctx: &Context) -> Result<Vec<NewsDto>, NewsUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_NEWS_ALL));

        let raw_news_list = self.repositories.news_repository().list().await?;
        let news_list = raw_news_list.into_iter().map(NewsDto::from_entity);
        Ok(news_list.collect())
    }

    pub async fn create(
        &self,
        ctx: &Context,
        raw_news: CreateNewsDto,
    ) -> Result<(), NewsUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::CREATE_NEWS));

        let news = raw_news.into_entity()?;
        self.repositories.news_repository().create(news).await?;
        Ok(())
    }

    pub async fn find_by_id(&self, ctx: &Context, id: String) -> Result<NewsDto, NewsUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
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
        ctx: &Context,
        news_data: UpdateNewsDto,
    ) -> Result<(), NewsUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;

        let id = NewsId::try_from(news_data.id)?;
        let news = self
            .repositories
            .news_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(NewsUseCaseError::NotFound(id.clone()))?;

        if !news.value.is_visible_to(&actor) {
            return Err(NewsUseCaseError::NotFound(id));
        }
        if !news.value.is_updatable_by(&actor) {
            return Err(PermissionDeniedError.into());
        }

        let mut new_news = news.value;

        let new_title = NewsTitle::new(news_data.title);
        if new_news.title() != &new_title {
            new_news.set_title(&actor, new_title)?;
        }

        let new_body = NewsBody::new(news_data.body);
        if new_news.body() != &new_body {
            new_news.set_body(&actor, new_body)?;
        }

        let new_categories = news_data.categories.into_entity()?;
        if new_news.categories() != &new_categories {
            new_news.set_categories(&actor, new_categories)?;
        }

        let new_attributes = news_data.attributes.into_entity()?;
        if new_news.attributes() != &new_attributes {
            new_news.set_attributes(&actor, new_attributes)?;
        }

        self.repositories.news_repository().update(new_news).await?;
        Ok(())
    }

    pub async fn delete_by_id(&self, ctx: &Context, id: String) -> Result<(), NewsUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::{
        entity::{permission::PermissionDeniedError, user::UserRole},
        test::{fixture, repository::MockRepositories},
    };

    use crate::{
        context::Context,
        dto::news::{CreateNewsDto, UpdateNewsDto},
        interactor::news::{NewsUseCase, NewsUseCaseError},
    };

    #[tokio::test]
    async fn list_general_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_list()
            .returning(|| Ok(vec![fixture::date::with(fixture::news::news1())]));
        let use_case = NewsUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case.list(&ctx).await;
        assert!(matches!(res, Ok(_)));
    }

    #[tokio::test]
    async fn create_committee_fail() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = NewsUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .create(
                &ctx,
                CreateNewsDto::new(
                    fixture::news::title1().value(),
                    fixture::news::body1().value(),
                    fixture::news::categories1().value(),
                ),
            )
            .await;
        assert!(matches!(
            res,
            Err(NewsUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn create_operator_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_create()
            .returning(|_| Ok(()));
        let use_case = NewsUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .create(
                &ctx,
                CreateNewsDto::new(
                    fixture::news::title1().value(),
                    fixture::news::body1().value(),
                    fixture::news::categories1().value(),
                ),
            )
            .await;
        assert!(matches!(res, Ok(_)));
    }

    #[tokio::test]
    async fn find_by_id_general_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::news::news1()))));
        let use_case = NewsUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::news::id1().value().to_string())
            .await;
        assert!(matches!(res, Ok(_)));
    }

    #[tokio::test]
    async fn update_committee_fail() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::news::news1()))));
        repositories
            .news_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = NewsUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .update(
                &ctx,
                UpdateNewsDto::new(
                    fixture::news::id1().value().to_string(),
                    fixture::news::title2().value(),
                    fixture::news::body2().value(),
                    fixture::news::categories2().value(),
                ),
            )
            .await;
        assert!(matches!(
            res,
            Err(NewsUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn update_operator_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::news::news1()))));
        repositories
            .news_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let use_case = NewsUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .update(
                &ctx,
                UpdateNewsDto::new(
                    fixture::news::id1().value().to_string(),
                    fixture::news::title2().value(),
                    fixture::news::body2().value(),
                    fixture::news::categories2().value(),
                ),
            )
            .await;
        assert!(matches!(res, Ok(_)));
    }

    #[tokio::test]
    async fn delete_by_id_committee_fail() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::news::news1()))));
        repositories
            .news_repository_mut()
            .expect_delete_by_id()
            .returning(|_| Ok(()));
        let use_case = NewsUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .delete_by_id(&ctx, fixture::news::id1().value().to_string())
            .await;
        assert!(matches!(
            res,
            Err(NewsUseCaseError::PermissionDeniedError(
                PermissionDeniedError
            ))
        ));
    }

    #[tokio::test]
    async fn delete_by_id_operator_success() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::news::news1()))));
        repositories
            .news_repository_mut()
            .expect_delete_by_id()
            .returning(|_| Ok(()));
        let use_case = NewsUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .delete_by_id(&ctx, fixture::news::id1().value().to_string())
            .await;
        assert!(matches!(res, Ok(_)));
    }
}
