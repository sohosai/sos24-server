use sos24_domain::entity::{
    common::date::WithDate,
    news::{News, NewsBody, NewsCategories, NewsTitle},
};

#[derive(Debug)]
pub struct CreateNewsDto {
    pub title: String,
    pub body: String,
    pub categories: i32,
}

impl CreateNewsDto {
    pub fn new(title: String, body: String, categories: i32) -> Self {
        Self {
            title,
            body,
            categories,
        }
    }
}

impl From<CreateNewsDto> for News {
    fn from(news: CreateNewsDto) -> Self {
        News::new(
            NewsTitle::new(news.title),
            NewsBody::new(news.body),
            NewsCategories::new(news.categories),
        )
    }
}

#[derive(Debug)]
pub struct UpdateNewsDto {
    pub id: String,
    pub title: String,
    pub body: String,
    pub categories: i32,
}

impl UpdateNewsDto {
    pub fn new(id: String, title: String, body: String, categories: i32) -> Self {
        Self {
            id,
            title,
            body,
            categories,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NewsDto {
    pub id: String,
    pub title: String,
    pub body: String,
    pub categories: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<WithDate<News>> for NewsDto {
    fn from(news: WithDate<News>) -> Self {
        Self {
            id: news.value.id.value().to_string(),
            title: news.value.title.value(),
            body: news.value.body.value(),
            categories: news.value.categories.value(),
            created_at: news.created_at,
            updated_at: news.updated_at,
            deleted_at: news.deleted_at,
        }
    }
}
