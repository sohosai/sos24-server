use std::sync::Arc;

use sos24_domain::ensure;
use sos24_domain::entity::file_data::FileId;
use sos24_domain::entity::project::{ProjectAttributes, ProjectCategories};
use sos24_domain::repository::file_data::FileDataRepository;
use sos24_domain::{
    entity::news::{NewsBody, NewsId, NewsTitle},
    repository::{news::NewsRepository, Repositories},
};

use crate::context::Context;
use crate::project::dto::{ProjectAttributesDto, ProjectCategoriesDto};

use super::{NewsUseCase, NewsUseCaseError};

pub struct UpdateNewsCommand {
    pub title: String,
    pub body: String,
    pub attachments: Vec<String>,
    pub categories: ProjectCategoriesDto,
    pub attributes: ProjectAttributesDto,
}

impl<R: Repositories> NewsUseCase<R> {
    pub async fn update(
        &self,
        ctx: &Context,
        news_id: String,
        news_data: UpdateNewsCommand,
    ) -> Result<(), NewsUseCaseError> {
        let actor = ctx.actor(Arc::clone(&self.repositories)).await?;

        let id = NewsId::try_from(news_id)?;
        let news = self
            .repositories
            .news_repository()
            .find_by_id(id.clone())
            .await?
            .ok_or(NewsUseCaseError::NotFound(id.clone()))?;

        ensure!(news.value.is_visible_to(&actor));
        ensure!(news.value.is_updatable_by(&actor));

        let mut new_news = news.value;
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
        context::Context,
        news::use_case::{update::UpdateNewsCommand, NewsUseCase, NewsUseCaseError},
        project::dto::{ProjectAttributesDto, ProjectCategoriesDto},
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
        let use_case = NewsUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::Committee));
        let res = use_case
            .update(
                &ctx,
                fixture::news::id1().value().to_string(),
                UpdateNewsCommand {
                    title: fixture::news::title2().value(),
                    body: fixture::news::body2().value(),
                    attachments: fixture::news::attachments2()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    categories: ProjectCategoriesDto::from(fixture::news::categories2()),
                    attributes: ProjectAttributesDto::from(fixture::news::attributes2()),
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
        let use_case = NewsUseCase::new(Arc::new(repositories));

        let ctx = Context::with_actor(fixture::actor::actor1(UserRole::CommitteeOperator));
        let res = use_case
            .update(
                &ctx,
                fixture::news::id1().value().to_string(),
                UpdateNewsCommand {
                    title: fixture::news::title2().value(),
                    body: fixture::news::body2().value(),
                    attachments: fixture::news::attachments2()
                        .into_iter()
                        .map(|id| id.value().to_string())
                        .collect(),
                    categories: ProjectCategoriesDto::from(fixture::news::categories2()),
                    attributes: ProjectAttributesDto::from(fixture::news::attributes2()),
                },
            )
            .await;
        assert!(res.is_ok());
    }
}
