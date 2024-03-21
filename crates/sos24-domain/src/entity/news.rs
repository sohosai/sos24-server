use getset::Getters;
use thiserror::Error;

use crate::{ensure, impl_value_object};

use super::{
    actor::Actor,
    permission::{PermissionDeniedError, Permissions},
};

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct News {
    #[getset(get = "pub")]
    id: NewsId,
    #[getset(get = "pub")]
    title: NewsTitle,
    #[getset(get = "pub")]
    body: NewsBody,
    #[getset(get = "pub")]
    categories: NewsCategories,
}

impl News {
    pub fn new(id: NewsId, title: NewsTitle, body: NewsBody, categories: NewsCategories) -> Self {
        Self {
            id,
            title,
            body,
            categories,
        }
    }

    pub fn create(title: NewsTitle, body: NewsBody, categories: NewsCategories) -> Self {
        Self {
            id: NewsId::new(uuid::Uuid::new_v4()),
            title,
            body,
            categories,
        }
    }

    pub fn destruct(self) -> DestructedNews {
        DestructedNews {
            id: self.id,
            title: self.title,
            body: self.body,
            categories: self.categories,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructedNews {
    pub id: NewsId,
    pub title: NewsTitle,
    pub body: NewsBody,
    pub categories: NewsCategories,
}

impl News {
    pub fn is_visible_to(&self, actor: &Actor) -> bool {
        actor.has_permission(Permissions::READ_NEWS_ALL)
    }

    pub fn is_updatable_by(&self, actor: &Actor) -> bool {
        actor.has_permission(Permissions::UPDATE_NEWS_ALL)
    }

    pub fn set_title(
        &mut self,
        actor: &Actor,
        title: NewsTitle,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.title = title;
        Ok(())
    }

    pub fn set_body(&mut self, actor: &Actor, body: NewsBody) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.body = body;
        Ok(())
    }

    pub fn set_categories(
        &mut self,
        actor: &Actor,
        categories: NewsCategories,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.categories = categories;
        Ok(())
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewsCategories(u32);

bitflags! {
    impl NewsCategories: u32 {
        const GENERAL = 1 << 0;
        const FOODS_WITH_KITCHEN = 1 << 1;
        const FOODS_WITHOUT_KITCHEN = 1 << 2;
        const FOODS_WITHOUT_COOKING = 1 << 3;
        const STAGE_1A = 1 << 4;
        const STAGE_UNIVERSITY_HALL = 1 << 5;
        const STAGE_UNITED = 1 << 6;
    }
}
