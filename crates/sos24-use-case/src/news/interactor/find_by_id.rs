use sos24_domain::{
    ensure,
    entity::{news::NewsId, permission::Permissions},
    repository::{news::NewsRepository, Repositories},
};

use crate::{
    news::{dto::NewsDto, NewsUseCase, NewsUseCaseError},
    shared::adapter::Adapters,
    shared::context::ContextProvider,
};

impl<R: Repositories, A: Adapters> NewsUseCase<R, A> {
    pub async fn find_by_id(
        &self,
        ctx: &impl ContextProvider,
        id: String,
    ) -> Result<NewsDto, NewsUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;
        ensure!(actor.has_permission(Permissions::READ_NEWS_ALL)); // first check of visibility

        let id = NewsId::try_from(id)?;
        let raw_news = self
            .repositories
            .news_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(NewsUseCaseError::NotFound(id.clone()))?;

        // check visibility for each state of news
        // and for each user role
        if !raw_news.is_visible_to(&actor) {
            return Err(NewsUseCaseError::NotFound(id));
        }

        Ok(NewsDto::from(raw_news))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::{
        entity::user::UserRole,
        test::{fixture, repository::MockRepositories},
    };

    use crate::{news::NewsUseCase, shared::adapter::MockAdapters, shared::context::TestContext};

    #[tokio::test]
    async fn 一般ユーザーはお知らせを取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::news::news1())));
        let adapters = MockAdapters::default();
        let use_case = NewsUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case
            .find_by_id(&ctx, fixture::news::id1().value().to_string())
            .await;
        assert!(res.is_ok());
    }
}
