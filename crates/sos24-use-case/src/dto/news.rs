use sos24_domain::entity::news::{News, NewsBody, NewsCategories, NewsId, NewsTitle};

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
            NewsId::new(uuid::Uuid::new_v4()),
            NewsTitle::new(news.title),
            NewsBody::new(news.body),
            NewsCategories::new(news.categories),
            chrono::Utc::now(),
            chrono::Utc::now(),
            None,
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

impl From<News> for NewsDto {
    fn from(value: News) -> Self {
        Self {
            id: value.id.value().to_string(),
            title: value.title.value(),
            body: value.body.value(),
            categories: value.categories.value(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}
