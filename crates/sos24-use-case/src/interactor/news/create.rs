use std::sync::Arc;

use sos24_domain::{
    ensure,
    entity::permission::Permissions,
    repository::{file_data::FileDataRepository, news::NewsRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{news::CreateNewsDto, ToEntity},
};

use super::{NewsUseCase, NewsUseCaseError};

impl<R: Repositories> NewsUseCase<R> {
    pub async fn create(
        &self,
        ctx: &Context,
        raw_news: CreateNewsDto,
    ) -> Result<(), NewsUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;
        ensure!(actor.has_permission(Permissions::CREATE_NEWS));

        let news = raw_news.into_entity()?;
        for file_id in news.attachments() {
            let _ = self
                .repositories
                .file_data_repository()
                .find_by_id(file_id.clone())
                .await?
                .ok_or(NewsUseCaseError::FileNotFound(file_id.clone()))?;
        }

        self.repositories.news_repository().create(news).await?;
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
        dto::{news::CreateNewsDto, FromEntity},
        interactor::news::{NewsUseCase, NewsUseCaseError},
    };

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
                    fixture::news::attachments1()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    Vec::from_entity(fixture::news::categories1()),
                    Vec::from_entity(fixture::news::attributes1()),
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
                    fixture::news::attachments1()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    Vec::from_entity(fixture::news::categories1()),
                    Vec::from_entity(fixture::news::attributes1()),
                ),
            )
            .await;
        assert!(matches!(res, Ok(_)));
    }
}
