use serde::{Deserialize, Serialize};

use sos24_use_case::dto::news::{CreateNewsDto, NewsDto, UpdateNewsDto};
use sos24_use_case::dto::project::{ProjectAttributeDto, ProjectCategoryDto};

use crate::model::project::{ProjectAttribute, ProjectCategory};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNews {
    title: String,
    body: String,
    attachments: Vec<String>,
    categories: Vec<ProjectCategory>,
    attributes: Vec<ProjectAttribute>,
}

impl From<CreateNews> for CreateNewsDto {
    fn from(news: CreateNews) -> Self {
        CreateNewsDto::new(
            news.title,
            news.body,
            news.attachments,
            news.categories
                .into_iter()
                .map(ProjectCategoryDto::from)
                .collect(),
            news.attributes
                .into_iter()
                .map(ProjectAttributeDto::from)
                .collect(),
        )
    }
}

#[derive(Debug, Serialize)]
pub struct CreatedNews {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNews {
    title: String,
    body: String,
    attachments: Vec<String>,
    categories: Vec<ProjectCategory>,
    attributes: Vec<ProjectAttribute>,
}

pub trait ConvertToUpdateNewsDto {
    fn to_update_news_dto(self) -> UpdateNewsDto;
}

impl ConvertToUpdateNewsDto for (String, UpdateNews) {
    fn to_update_news_dto(self) -> UpdateNewsDto {
        let (id, news) = self;
        UpdateNewsDto::new(
            id,
            news.title,
            news.body,
            news.attachments,
            news.categories
                .into_iter()
                .map(ProjectCategoryDto::from)
                .collect(),
            news.attributes
                .into_iter()
                .map(ProjectAttributeDto::from)
                .collect(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct News {
    pub id: String,
    pub title: String,
    pub body: String,
    pub attachments: Vec<String>,
    pub categories: Vec<ProjectCategory>,
    pub attributes: Vec<ProjectAttribute>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<NewsDto> for News {
    fn from(news: NewsDto) -> Self {
        News {
            id: news.id,
            title: news.title,
            body: news.body,
            attachments: news.attachments,
            categories: news
                .categories
                .into_iter()
                .map(ProjectCategory::from)
                .collect(),
            attributes: news
                .attributes
                .into_iter()
                .map(ProjectAttribute::from)
                .collect(),
            created_at: news.created_at.to_rfc3339(),
            updated_at: news.updated_at.to_rfc3339(),
            deleted_at: news.deleted_at.map(|it| it.to_rfc3339()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NewsSummary {
    id: String,
    title: String,
    categories: Vec<ProjectCategory>,
    attributes: Vec<ProjectAttribute>,
    updated_at: String,
}

impl From<NewsDto> for NewsSummary {
    fn from(news: NewsDto) -> Self {
        NewsSummary {
            id: news.id,
            title: news.title,
            attributes: news
                .attributes
                .into_iter()
                .map(ProjectAttribute::from)
                .collect(),
            categories: news
                .categories
                .into_iter()
                .map(ProjectCategory::from)
                .collect(),
            updated_at: news.updated_at.to_rfc3339(),
        }
    }
}
