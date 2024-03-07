use thiserror::Error;

use crate::impl_value_object;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct News {
    pub id: NewsId,
    pub title: NewsTitle,
    pub body: NewsBody,
    pub categories: NewsCategories,
}

impl News {
    pub fn new(title: NewsTitle, body: NewsBody, categories: NewsCategories) -> Self {
        Self {
            id: NewsId::new(uuid::Uuid::new_v4()),
            title,
            body,
            categories,
        }
    }
}

impl_value_object!(NewsId(uuid::Uuid));
#[derive(Debug, Error)]
pub enum NewsIdError {
    #[error("Invalid UUID")]
    InvalidUuid,
}
impl TryFrom<String> for NewsId {
    type Error = NewsIdError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let uuid = uuid::Uuid::parse_str(&value).map_err(|_| NewsIdError::InvalidUuid)?;
        Ok(Self(uuid))
    }
}

impl_value_object!(NewsTitle(String));
impl_value_object!(NewsBody(String));
impl_value_object!(NewsCategories(i32));
