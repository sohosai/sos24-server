use sos24_domain::entity::news::{News, NewsState};

use crate::project::dto::{ProjectAttributesDto, ProjectCategoriesDto};

#[derive(Debug)]
pub struct NewsDto {
    pub id: String,
    pub state: NewsStateDto,
    pub title: String,
    pub body: String,
    pub attachments: Vec<String>,
    pub categories: ProjectCategoriesDto,
    pub attributes: ProjectAttributesDto,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub scheduled_at: Option<String>,
}

#[derive(Debug)]
pub enum NewsStateDto {
    Draft,
    Scheduled,
    Published,
}

impl NewsStateDto {
    pub fn from_news_state(state: NewsState) -> (Self, Option<String>) {
        match state {
            NewsState::Draft => (NewsStateDto::Draft, None),
            NewsState::Scheduled(date) => {
                (NewsStateDto::Scheduled, Some(date.value().to_rfc3339()))
            }
            NewsState::Published => (NewsStateDto::Published, None),
        }
    }
}

impl From<News> for NewsDto {
    fn from(news: News) -> Self {
        let news = news.destruct();
        let (state, scheduled_at) = NewsStateDto::from_news_state(news.state);
        Self {
            id: news.id.value().to_string(),
            state,
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
            scheduled_at,
        }
    }
}
