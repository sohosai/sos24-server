use serde::{Deserialize, Serialize};
use sos24_use_case::dto::news_attachment::{CreateNewsAttachmentDto, NewsAttachmentDto};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNewsAttachment {
    news_id: String,
    url: String,
}

impl From<CreateNewsAttachment> for CreateNewsAttachmentDto {
    fn from(news_attachment: CreateNewsAttachment) -> Self {
        CreateNewsAttachmentDto::new(news_attachment.news_id, news_attachment.url)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsAttachment {
    pub id: String,
    pub news_id: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<NewsAttachmentDto> for NewsAttachment {
    fn from(news_attachment: NewsAttachmentDto) -> Self {
        NewsAttachment {
            id: news_attachment.id,
            news_id: news_attachment.news_id,
            url: news_attachment.url,
            created_at: news_attachment.created_at.to_rfc3339(),
            updated_at: news_attachment.updated_at.to_rfc3339(),
            deleted_at: news_attachment.deleted_at.map(|it| it.to_rfc3339()),
        }
    }
}
