use std::sync::Arc;

use sos24_domain::entity::file_data::FileId;
use sos24_domain::repository::file_data::FileDataRepository;
use sos24_domain::{
    entity::{
        news::{NewsBody, NewsId, NewsTitle},
        permission::PermissionDeniedError,
    },
    repository::{news::NewsRepository, Repositories},
};

use crate::{
    context::Context,
    dto::{news::UpdateNewsDto, ToEntity},
};

use super::{NewsUseCase, NewsUseCaseError};

impl<R: Repositories> NewsUseCase<R> {
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

        let new_attachments = news_data
            .attachments
            .into_iter()
            .map(FileId::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        if new_news.attachments() != &new_attachments {
            for file_id in &new_attachments {
                let _ = self
                    .repositories
                    .file_data_repository()
                    .find_by_id(file_id.clone())
                    .await?
                    .ok_or(NewsUseCaseError::FileNotFound(file_id.clone()))?;
            }

            new_news.set_attachments(&actor, new_attachments)?;
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
        dto::{news::UpdateNewsDto, FromEntity},
        interactor::news::{NewsUseCase, NewsUseCaseError},
    };

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
                    fixture::news::attachments2()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    Vec::from_entity(fixture::news::categories2()),
                    Vec::from_entity(fixture::news::attributes2()),
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
                    fixture::news::attachments2()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    Vec::from_entity(fixture::news::categories2()),
                    Vec::from_entity(fixture::news::attributes2()),
                ),
            )
            .await;
        assert!(matches!(res, Ok(_)));
    }
}
