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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::{
        entity::{permission::PermissionDeniedError, user::UserRole},
        test::{fixture, repository::MockRepositories},
    };

    use crate::{
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

        let actor = fixture::actor::actor1(UserRole::General);
        let res = use_case.list(&actor).await;
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

        let actor = fixture::actor::actor1(UserRole::Committee);
        let res = use_case
            .create(
                &actor,
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

        let actor = fixture::actor::actor1(UserRole::CommitteeOperator);
        let res = use_case
            .create(
                &actor,
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

        let actor = fixture::actor::actor1(UserRole::General);
        let res = use_case
            .find_by_id(&actor, fixture::news::id1().value().to_string())
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

        let actor = fixture::actor::actor1(UserRole::Committee);
        let res = use_case
            .update(
                &actor,
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

        let actor = fixture::actor::actor1(UserRole::CommitteeOperator);
        let res = use_case
            .update(
                &actor,
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

        let actor = fixture::actor::actor1(UserRole::Committee);
        let res = use_case
            .delete_by_id(&actor, fixture::news::id1().value().to_string())
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

        let actor = fixture::actor::actor1(UserRole::CommitteeOperator);
        let res = use_case
            .delete_by_id(&actor, fixture::news::id1().value().to_string())
            .await;
        assert!(matches!(res, Ok(_)));
    }
}
