use sos24_domain::{
    entity::{news::NewsId, permission::PermissionDeniedError},
    repository::{news::NewsRepository, Repositories},
};

use crate::{
    news::{NewsUseCase, NewsUseCaseError},
    shared::adapter::Adapters,
    shared::context::ContextProvider,
};

impl<R: Repositories, A: Adapters> NewsUseCase<R, A> {
    pub async fn delete_by_id(
        &self,
        ctx: &impl ContextProvider,
        id: String,
    ) -> Result<(), NewsUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;

        let id = NewsId::try_from(id)?;
        let news = self
            .repositories
            .news_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(NewsUseCaseError::NotFound(id.clone()))?;

        // check deletability for each news state
        // and for each user role
        if !news.is_deletable_by(&actor) {
            return Err(PermissionDeniedError.into());
        }

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
        news::{NewsUseCase, NewsUseCaseError},
        shared::adapter::MockAdapters,
        shared::context::TestContext,
    };

    #[tokio::test]
    async fn 実委人はお知らせを削除できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::news::news1())));
        repositories
            .news_repository_mut()
            .expect_delete_by_id()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = NewsUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
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
    async fn 実委人管理者はお知らせを削除できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::news::news1())));
        repositories
            .news_repository_mut()
            .expect_delete_by_id()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = NewsUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .delete_by_id(&ctx, fixture::news::id1().value().to_string())
            .await;
        assert!(res.is_ok());
    }
}
