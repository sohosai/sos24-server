use sos24_domain::entity::news::News;

use crate::project::dto::{ProjectAttributesDto, ProjectCategoriesDto};

#[derive(Debug)]
pub struct NewsDto {
    pub id: String,
    pub title: String,
    pub body: String,
    pub attachments: Vec<String>,
    pub categories: ProjectCategoriesDto,
    pub attributes: ProjectAttributesDto,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<News> for NewsDto {
    fn from(news: News) -> Self {
        let news = news.destruct();
        Self {
            id: news.id.value().to_string(),
            title: news.title.value(),
            body: news.body.value(),
            attachments: news
                .attachments
                .into_iter()
                .map(|file_id| file_id.value().to_string())
                .collect(),
            categories: ProjectCategoriesDto::from(news.categories),
            attributes: ProjectAttributesDto::from(news.attributes),
            created_at: news.created_at.value(),
            updated_at: news.updated_at.value(),
        }
    }
}
