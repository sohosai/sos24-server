use serde::{Deserialize, Serialize};
use sos24_use_case::dto::news::{CreateNewsDto, NewsDto, UpdateNewsDto};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNews {
    title: String,
    body: String,
    categories: i32,
}

impl From<CreateNews> for CreateNewsDto {
    fn from(news: CreateNews) -> Self {
        CreateNewsDto::new(news.title, news.body, news.categories)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNews {
    title: String,
    body: String,
    categories: i32,
}

pub trait ConvertToUpdateNewsDto {
    fn to_update_news_dto(self) -> UpdateNewsDto;
}

impl ConvertToUpdateNewsDto for (String, UpdateNews) {
    fn to_update_news_dto(self) -> UpdateNewsDto {
        let (id, news) = self;
        UpdateNewsDto::new(id, news.title, news.body, news.categories)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct News {
    pub id: String,
    pub title: String,
    pub body: String,
    pub categories: i32,
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
            categories: news.categories,
            created_at: news.created_at.to_rfc3339(),
            updated_at: news.updated_at.to_rfc3339(),
            deleted_at: news.deleted_at.map(|it| it.to_rfc3339()),
        }
    }
}
