use serde::{Deserialize, Serialize};

use sos24_use_case::news::dto::NewsDto;
use sos24_use_case::news::interactor::create::CreateNewsCommand;
use sos24_use_case::news::interactor::update::UpdateNewsCommand;
use sos24_use_case::project::dto::{ProjectAttributesDto, ProjectCategoriesDto};
use utoipa::ToSchema;

use super::project::{ProjectAttributes, ProjectCategories};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateNews {
    title: String,
    body: String,
    #[schema(format = "uuid")]
    attachments: Vec<String>,
    categories: ProjectCategories,
    attributes: ProjectAttributes,
}

impl From<CreateNews> for CreateNewsCommand {
    fn from(news: CreateNews) -> Self {
        CreateNewsCommand {
            title: news.title,
            body: news.body,
            attachments: news.attachments,
            categories: ProjectCategoriesDto::from(news.categories),
            attributes: ProjectAttributesDto::from(news.attributes),
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
    title: String,
    body: String,
    #[schema(format = "uuid")]
    attachments: Vec<String>,
    categories: ProjectCategories,
    attributes: ProjectAttributes,
}

pub trait ConvertToUpdateNewsDto {
    fn to_update_news_dto(self) -> UpdateNewsCommand;
}

impl ConvertToUpdateNewsDto for (String, UpdateNews) {
    fn to_update_news_dto(self) -> UpdateNewsCommand {
        let (id, news) = self;
        UpdateNewsCommand {
            id,
            title: news.title,
            body: news.body,
            attachments: news.attachments,
            categories: ProjectCategoriesDto::from(news.categories),
            attributes: ProjectAttributesDto::from(news.attributes),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct News {
    #[schema(format = "uuid")]
    pub id: String,
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
}

impl From<NewsDto> for News {
    fn from(news: NewsDto) -> Self {
        News {
            id: news.id,
            title: news.title,
            body: news.body,
            attachments: news.attachments,
            categories: ProjectCategories::from(news.categories),
            attributes: ProjectAttributes::from(news.attributes),
            created_at: news.created_at.to_rfc3339(),
            updated_at: news.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct NewsSummary {
    #[schema(format = "uuid")]
    id: String,
    title: String,
    categories: ProjectCategories,
    attributes: ProjectAttributes,
    #[schema(format = "date-time")]
    updated_at: String,
}

impl From<NewsDto> for NewsSummary {
    fn from(news: NewsDto) -> Self {
        NewsSummary {
            id: news.id,
            title: news.title,
            categories: ProjectCategories::from(news.categories),
            attributes: ProjectAttributes::from(news.attributes),
            updated_at: news.updated_at.to_rfc3339(),
        }
    }
}
