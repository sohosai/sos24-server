use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{news::NewsRepository, Repositories},
};

use crate::{
    adapter::Adapters,
    context::ContextProvider,
    dto::{news::NewsDto, FromEntity},
};

use super::{NewsUseCase, NewsUseCaseError};

impl<R: Repositories, A: Adapters> NewsUseCase<R, A> {
    pub async fn list(&self, ctx: &impl ContextProvider) -> Result<Vec<NewsDto>, NewsUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::READ_NEWS_ALL));

        let raw_news_list = self.repositories.news_repository().list().await?;
        let news_list = raw_news_list.into_iter().map(NewsDto::from_entity);
        Ok(news_list.collect())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::{
        entity::user::UserRole,
        test::{fixture, repository::MockRepositories},
    };

    use crate::{adapter::MockAdapters, context::TestContext, interactor::news::NewsUseCase};

    #[tokio::test]
    async fn 一般ユーザーはお知らせ一覧を取得できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_list()
            .returning(|| Ok(vec![fixture::date::with(fixture::news::news1())]));
        let adapters = MockAdapters::default();
        let use_case = NewsUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::General));
        let res = use_case.list(&ctx).await;
        assert!(res.is_ok());
    }
}
