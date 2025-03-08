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
}

#[derive(Debug)]
pub enum NewsStateDto {
    Draft,
    Scheduled(chrono::DateTime<chrono::Utc>),
    Published,
}

impl From<NewsState> for NewsStateDto {
    fn from(value: NewsState) -> NewsStateDto {
        match value {
            NewsState::Draft => NewsStateDto::Draft,
            NewsState::Scheduled(date) => NewsStateDto::Scheduled(date.value()),
            NewsState::Published => NewsStateDto::Published,
        }
    }
}
// impl From<NewsStateDto> for NewsState {
//     fn from(value: NewsStateDto) -> NewsState {
//         match value {
//             NewsStateDto::Draft => NewsState::Draft,
//             NewsStateDto::Scheduled(date) => NewsState::Scheduled(datetime::DateTime::new(date)),
//             NewsStateDto::Published => NewsState::Published,
//         }
//     }
// }

impl From<News> for NewsDto {
    fn from(news: News) -> Self {
        let news = news.destruct();
        Self {
            id: news.id.value().to_string(),
            state: NewsStateDto::from(news.state),
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
