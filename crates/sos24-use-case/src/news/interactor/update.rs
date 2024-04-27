use sos24_domain::entity::file_data::FileId;
use sos24_domain::entity::news::News;
use sos24_domain::repository::file_data::FileDataRepository;
use sos24_domain::{
    entity::{
        news::{NewsBody, NewsId, NewsTitle},
        permission::PermissionDeniedError,
    },
    repository::{news::NewsRepository, Repositories},
};

use crate::news::{NewsUseCase, NewsUseCaseError};
use crate::project::dto::{ProjectAttributeDto, ProjectCategoryDto};
use crate::shared::adapter::Adapters;
use crate::shared::context::ContextProvider;
use crate::ToEntity;

#[derive(Debug)]
pub struct UpdateNewsCommand {
    pub id: String,
    pub title: String,
    pub body: String,
    pub attachments: Vec<String>,
    pub categories: Vec<ProjectCategoryDto>,
    pub attributes: Vec<ProjectAttributeDto>,
}

impl ToEntity for UpdateNewsCommand {
    type Entity = News;
    type Error = NewsUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        Ok(News::new(
            NewsId::try_from(self.id)?,
            NewsTitle::new(self.title),
            NewsBody::new(self.body),
            self.attachments
                .into_iter()
                .map(FileId::try_from)
                .collect::<Result<_, _>>()?,
            self.categories.into_entity()?,
            self.attributes.into_entity()?,
        ))
    }
}

impl<R: Repositories, A: Adapters> NewsUseCase<R, A> {
    pub async fn update(
        &self,
        ctx: &impl ContextProvider,
        news_data: UpdateNewsCommand,
    ) -> Result<(), NewsUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;

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
        news::{interactor::update::UpdateNewsCommand, NewsUseCase, NewsUseCaseError},
        shared::{adapter::MockAdapters, context::TestContext},
        FromEntity,
    };

    #[tokio::test]
    async fn 実委人はお知らせを更新できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::news::news1()))));
        repositories
            .news_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = NewsUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .update(
                &ctx,
                UpdateNewsCommand {
                    id: fixture::news::id1().value().to_string(),
                    title: fixture::news::title2().value(),
                    body: fixture::news::body2().value(),
                    attachments: fixture::news::attachments2()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    categories: Vec::from_entity(fixture::news::categories2()),
                    attributes: Vec::from_entity(fixture::news::attributes2()),
                },
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
    async fn 実委人管理者はお知らせを更新できる() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::date::with(fixture::news::news1()))));
        repositories
            .news_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = NewsUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .update(
                &ctx,
                UpdateNewsCommand {
                    id: fixture::news::id1().value().to_string(),
                    title: fixture::news::title2().value(),
                    body: fixture::news::body2().value(),
                    attachments: fixture::news::attachments2()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    categories: Vec::from_entity(fixture::news::categories2()),
                    attributes: Vec::from_entity(fixture::news::attributes2()),
                },
            )
            .await;
        assert!(res.is_ok());
    }
}
