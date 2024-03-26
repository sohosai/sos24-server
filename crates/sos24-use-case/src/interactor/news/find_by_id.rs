use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::{news::NewsId, permission::Permissions},
    repository::{news::NewsRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{news::NewsDto, FromEntity},
};

use super::{NewsUseCase, NewsUseCaseError};

impl<R: Repositories> NewsUseCase<R> {
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
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sos24_domain::{
        entity::user::UserRole,
        test::{fixture, repository::MockRepositories},
    };

    use crate::{context::Context, interactor::news::NewsUseCase};

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
}
