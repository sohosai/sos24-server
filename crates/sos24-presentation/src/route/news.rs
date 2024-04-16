use serde::{Deserialize, Serialize};
use sos24_use_case::news::dto::NewsDto;

use super::project::{ProjectAttributes, ProjectCategories};

pub mod delete_by_id;
pub mod get;
pub mod get_by_id;
pub mod post;
pub mod put_by_id;

#[derive(Debug, Serialize, Deserialize)]
pub struct News {
    pub id: String,
    pub title: String,
    pub body: String,
    pub attachments: Vec<String>,
    pub categories: ProjectCategories,
    pub attributes: ProjectAttributes,
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
            categories: ProjectCategories::from(news.categories),
            attributes: ProjectAttributes::from(news.attributes),
            created_at: news.created_at.to_rfc3339(),
            updated_at: news.updated_at.to_rfc3339(),
            deleted_at: news.deleted_at.map(|it| it.to_rfc3339()),
        }
    }
}
