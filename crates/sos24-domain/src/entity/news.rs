use getset::Getters;
use thiserror::Error;

use crate::entity::file_data::FileId;
use crate::entity::project::{ProjectAttributes, ProjectCategories};
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
    attachments: Vec<FileId>,
    #[getset(get = "pub")]
    categories: ProjectCategories,
    #[getset(get = "pub")]
    attributes: ProjectAttributes,
}

impl News {
    pub fn new(
        id: NewsId,
        title: NewsTitle,
        body: NewsBody,
        attachments: Vec<FileId>,
        categories: ProjectCategories,
        attributes: ProjectAttributes,
    ) -> Self {
        Self {
            id,
            title,
            body,
            attachments,
            categories,
            attributes,
        }
    }

    pub fn create(
        title: NewsTitle,
        body: NewsBody,
        attachments: Vec<FileId>,
        categories: ProjectCategories,
        attributes: ProjectAttributes,
    ) -> Self {
        Self {
            id: NewsId::new(uuid::Uuid::new_v4()),
            title,
            body,
            attachments,
            categories,
            attributes,
        }
    }

    pub fn destruct(self) -> DestructedNews {
        DestructedNews {
            id: self.id,
            title: self.title,
            body: self.body,
            attachments: self.attachments,
            categories: self.categories,
            attributes: self.attributes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructedNews {
    pub id: NewsId,
    pub title: NewsTitle,
    pub body: NewsBody,
    pub attachments: Vec<FileId>,
    pub categories: ProjectCategories,
    pub attributes: ProjectAttributes,
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

    pub fn set_attachments(
        &mut self,
        actor: &Actor,
        attachments: Vec<FileId>,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.attachments = attachments;
        Ok(())
    }

    pub fn set_categories(
        &mut self,
        actor: &Actor,
        categories: ProjectCategories,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.categories = categories;
        Ok(())
    }

    pub fn set_attributes(
        &mut self,
        actor: &Actor,
        attributes: ProjectAttributes,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.attributes = attributes;
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
