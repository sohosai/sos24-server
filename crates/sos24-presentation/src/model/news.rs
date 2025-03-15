use serde::{Deserialize, Serialize};

use sos24_use_case::news::dto::{NewsDto, NewsStateDto};
use sos24_use_case::news::interactor::create::CreateNewsCommand;
use sos24_use_case::news::interactor::update::UpdateNewsCommand;
use sos24_use_case::project::dto::{ProjectAttributesDto, ProjectCategoriesDto};
use utoipa::ToSchema;

use super::project::{ProjectAttributes, ProjectCategories};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateNews {
    state: CreateNewsState,
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
pub enum CreateNewsState {
    Draft,
    Scheduled,
    Published,
}

impl From<CreateNewsState> for NewsStateDto {
    fn from(value: CreateNewsState) -> Self {
        match value {
            CreateNewsState::Draft => NewsStateDto::Draft,
            CreateNewsState::Scheduled => NewsStateDto::Scheduled,
            CreateNewsState::Published => NewsStateDto::Published,
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
    state: CreateNewsState,
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

pub trait ConvertToString {
    fn to_string(&self) -> String;
}

impl ConvertToString for NewsStateDto {
    fn to_string(&self) -> String {
        match self {
            NewsStateDto::Draft => "draft".to_string(),
            NewsStateDto::Scheduled => "scheduled".to_string(),
            NewsStateDto::Published => "published".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct News {
    #[schema(format = "uuid")]
    pub id: String,
    pub state: String,
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
            state: news.state.to_string(),
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
    state: String,
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
            state: news.state.to_string(),
            title: news.title,
            categories: ProjectCategories::from(news.categories),
            attributes: ProjectAttributes::from(news.attributes),
            updated_at: news.updated_at.to_rfc3339(),
            scheduled_at: news.scheduled_at,
        }
    }
}
