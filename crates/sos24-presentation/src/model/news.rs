use serde::{Deserialize, Serialize};

use sos24_use_case::news::dto::{NewsDto, NewsStateDto};
use sos24_use_case::news::interactor::create::CreateNewsCommand;
use sos24_use_case::news::interactor::update::UpdateNewsCommand;
use sos24_use_case::project::dto::{ProjectAttributesDto, ProjectCategoriesDto};
use utoipa::ToSchema;

use super::project::{ProjectAttributes, ProjectCategories};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateNews {
    state: NewsState,
    title: String,
    body: String,
    #[schema(format = "uuid")]
    attachments: Vec<String>,
    categories: ProjectCategories,
    attributes: ProjectAttributes,
    #[schema(format = "date-time")]
    scheduled_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum NewsState {
    Draft,
    Scheduled,
    Published,
}

impl From<NewsState> for NewsStateDto {
    fn from(value: NewsState) -> Self {
        match value {
            NewsState::Draft => NewsStateDto::Draft,
            NewsState::Scheduled => NewsStateDto::Scheduled,
            NewsState::Published => NewsStateDto::Published,
        }
    }
}

impl From<NewsStateDto> for NewsState {
    fn from(value: NewsStateDto) -> Self {
        match value {
            NewsStateDto::Draft => NewsState::Draft,
            NewsStateDto::Scheduled => NewsState::Scheduled,
            NewsStateDto::Published => NewsState::Published,
        }
    }
}

impl From<CreateNews> for CreateNewsCommand {
    fn from(news: CreateNews) -> Self {
        CreateNewsCommand {
            state: NewsStateDto::from(news.state),
            title: news.title,
            body: news.body,
            attachments: news.attachments,
            categories: ProjectCategoriesDto::from(news.categories),
            attributes: ProjectAttributesDto::from(news.attributes),
            scheduled_at: news.scheduled_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedNews {
    #[schema(format = "uuid")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateNews {
    state: NewsState,
    title: String,
    body: String,
    #[schema(format = "uuid")]
    attachments: Vec<String>,
    categories: ProjectCategories,
    attributes: ProjectAttributes,
    #[schema(format = "date-time")]
    scheduled_at: Option<String>,
}

pub trait ConvertToUpdateNewsDto {
    fn to_update_news_dto(self) -> UpdateNewsCommand;
}

impl ConvertToUpdateNewsDto for (String, UpdateNews) {
    fn to_update_news_dto(self) -> UpdateNewsCommand {
        let (id, news) = self;
        UpdateNewsCommand {
            id,
            state: NewsStateDto::from(news.state),
            title: news.title,
            body: news.body,
            attachments: news.attachments,
            categories: ProjectCategoriesDto::from(news.categories),
            attributes: ProjectAttributesDto::from(news.attributes),
            scheduled_at: news.scheduled_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct News {
    #[schema(format = "uuid")]
    pub id: String,
    pub state: NewsState,
    pub title: String,
    pub body: String,
    #[schema(format = "uuid")]
    pub attachments: Vec<String>,
    pub categories: ProjectCategories,
    pub attributes: ProjectAttributes,
    #[schema(format = "date-time")]
    pub created_at: String,
    #[schema(format = "date-time")]
    pub updated_at: String,
    #[schema(format = "date-time")]
    pub scheduled_at: Option<String>,
}

impl From<NewsDto> for News {
    fn from(news: NewsDto) -> Self {
        News {
            id: news.id,
            state: NewsState::from(news.state),
            title: news.title,
            body: news.body,
            attachments: news.attachments,
            categories: ProjectCategories::from(news.categories),
            attributes: ProjectAttributes::from(news.attributes),
            created_at: news.created_at.to_rfc3339(),
            updated_at: news.updated_at.to_rfc3339(),
            scheduled_at: news.scheduled_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct NewsSummary {
    #[schema(format = "uuid")]
    id: String,
    state: NewsState,
    title: String,
    categories: ProjectCategories,
    attributes: ProjectAttributes,
    #[schema(format = "date-time")]
    updated_at: String,
    #[schema(format = "date-time")]
    scheduled_at: Option<String>,
}

impl From<NewsDto> for NewsSummary {
    fn from(news: NewsDto) -> Self {
        NewsSummary {
            id: news.id,
            state: NewsState::from(news.state),
            title: news.title,
            categories: ProjectCategories::from(news.categories),
            attributes: ProjectAttributes::from(news.attributes),
            updated_at: news.updated_at.to_rfc3339(),
            scheduled_at: news.scheduled_at,
        }
    }
}
