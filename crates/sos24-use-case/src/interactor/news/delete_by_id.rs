use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{news::NewsId, permission::Permissions},
    repository::{news::NewsRepository, Repositories},
};

use crate::context::Context;

use super::{NewsUseCase, NewsUseCaseError};

impl<R: Repositories> NewsUseCase<R> {
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
        interactor::news::{NewsUseCase, NewsUseCaseError},
    };

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