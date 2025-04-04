use anyhow::anyhow;
use sos24_domain::entity::file_data::FileId;
use sos24_domain::entity::project::{ProjectAttributes, ProjectCategories};
use sos24_domain::repository::file_data::FileDataRepository;
use sos24_domain::{
    entity::{
        common::datetime::DateTime,
        news::{NewsBody, NewsId, NewsState, NewsTitle},
        permission::PermissionDeniedError,
    },
    repository::{news::NewsRepository, Repositories},
};

use crate::news::{dto::NewsStateDto, NewsUseCase, NewsUseCaseError};
use crate::project::dto::{ProjectAttributesDto, ProjectCategoriesDto};
use crate::shared::adapter::Adapters;
use crate::shared::context::ContextProvider;

#[derive(Debug)]
pub struct UpdateNewsCommand {
    pub id: String,
    pub state: NewsStateDto,
    pub title: String,
    pub body: String,
    pub attachments: Vec<String>,
    pub categories: ProjectCategoriesDto,
    pub attributes: ProjectAttributesDto,
    pub scheduled_at: Option<String>,
}

impl UpdateNewsCommand {
    pub fn get_news_state(&self) -> Result<NewsState, NewsUseCaseError> {
        match &self.state {
            NewsStateDto::Draft => Ok(NewsState::Draft),
            NewsStateDto::Scheduled => match &self.scheduled_at {
                Some(date) => Ok(NewsState::Scheduled(DateTime::try_from(date.clone())?)),
                None => Err(NewsUseCaseError::InternalError(anyhow!(
                    "Invalid newsstate format"
                ))),
            },
            NewsStateDto::Published => Ok(NewsState::Published),
        }
    }
}

impl<R: Repositories, A: Adapters> NewsUseCase<R, A> {
    pub async fn update(
        &self,
        ctx: &impl ContextProvider,
        news_data: UpdateNewsCommand,
    ) -> Result<(), NewsUseCaseError> {
        let actor = ctx.actor(&*self.repositories).await?;

        let id = NewsId::try_from(news_data.id.clone())?;
        let news = self
            .repositories
            .news_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(NewsUseCaseError::NotFound(id.clone()))?;

        if !news.is_visible_to(&actor) {
            return Err(NewsUseCaseError::NotFound(id));
        }

        let new_state = news_data.get_news_state()?;
        if !news.is_updatable_by(&actor, &new_state) {
            return Err(PermissionDeniedError.into());
        }

        let mut new_news = news;

        new_news.set_state(&actor, new_state)?;
        new_news.set_title(&actor, NewsTitle::new(news_data.title))?;
        new_news.set_body(&actor, NewsBody::new(news_data.body))?;

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

        new_news.set_categories(&actor, ProjectCategories::from(news_data.categories))?;
        new_news.set_attributes(&actor, ProjectAttributes::from(news_data.attributes))?;

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
        news::{
            dto::NewsStateDto, interactor::update::UpdateNewsCommand, NewsUseCase, NewsUseCaseError,
        },
        project::dto::{ProjectAttributesDto, ProjectCategoriesDto},
        shared::{adapter::MockAdapters, context::TestContext},
    };

    #[tokio::test]
    async fn 実委人はお知らせを更新できない() {
        let mut repositories = MockRepositories::default();
        repositories
            .news_repository_mut()
            .expect_find_by_id()
            .returning(|_| Ok(Some(fixture::news::news1())));
        repositories
            .news_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = NewsUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeViewer));
        let (state, scheduled_at) = NewsStateDto::from_news_state(fixture::news::state1());
        let res = use_case
            .update(
                &ctx,
                UpdateNewsCommand {
                    id: fixture::news::id1().value().to_string(),
                    state,
                    title: fixture::news::title2().value(),
                    body: fixture::news::body2().value(),
                    attachments: fixture::news::attachments2()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    categories: ProjectCategoriesDto::from(fixture::news::categories2()),
                    attributes: ProjectAttributesDto::from(fixture::news::attributes2()),
                    scheduled_at,
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
            .returning(|_| Ok(Some(fixture::news::news1())));
        repositories
            .news_repository_mut()
            .expect_update()
            .returning(|_| Ok(()));
        let adapters = MockAdapters::default();
        let use_case = NewsUseCase::new(Arc::new(repositories), Arc::new(adapters));

        let ctx = TestContext::new(fixture::actor::actor1(UserRole::CommitteeOperator));
        let (state, scheduled_at) = NewsStateDto::from_news_state(fixture::news::state1());
        let res = use_case
            .update(
                &ctx,
                UpdateNewsCommand {
                    id: fixture::news::id1().value().to_string(),
                    state,
                    title: fixture::news::title2().value(),
                    body: fixture::news::body2().value(),
                    attachments: fixture::news::attachments2()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    categories: ProjectCategoriesDto::from(fixture::news::categories2()),
                    attributes: ProjectAttributesDto::from(fixture::news::attributes2()),
                    scheduled_at,
                },
            )
            .await;
        assert!(res.is_ok());
    }
}
