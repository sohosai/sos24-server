use sos24_domain::entity::{common::date::WithDate, news::News};

use crate::project::dto::{ProjectAttributesDto, ProjectCategoriesDto};

pub struct NewsDto {
    pub id: String,
    pub title: String,
    pub body: String,
    pub attachments: Vec<String>,
    pub categories: ProjectCategoriesDto,
    pub attributes: ProjectAttributesDto,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<WithDate<News>> for NewsDto {
    fn from(entity: WithDate<News>) -> Self {
        let news = entity.value.destruct();
        Self {
            id: news.id.value().to_string(),
            title: news.title.value().to_string(),
            body: news.body.value().to_string(),
            attachments: news
                .attachments
                .into_iter()
                .map(|it| it.value().to_string())
                .collect(),
            categories: ProjectCategoriesDto::from(news.categories),
            attributes: ProjectAttributesDto::from(news.attributes),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}
