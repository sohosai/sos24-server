use std::str::FromStr;

use getset::Getters;
use thiserror::Error;

use crate::entity::file_data::FileId;
use crate::entity::form::{FormId, FormItemId};
use crate::entity::project::ProjectId;
use crate::{ensure, impl_value_object};

use super::actor::Actor;
use super::common::datetime::DateTime;
use super::permission::{PermissionDeniedError, Permissions};

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct FormAnswer {
    #[getset(get = "pub")]
    id: FormAnswerId,
    #[getset(get = "pub")]
    project_id: ProjectId,
    #[getset(get = "pub")]
    form_id: FormId,
    #[getset(get = "pub")]
    items: Vec<FormAnswerItem>,
    #[getset(get = "pub")]
    created_at: DateTime,
    #[getset(get = "pub")]
    updated_at: DateTime,
}

impl FormAnswer {
    pub fn create(project_id: ProjectId, form_id: FormId, items: Vec<FormAnswerItem>) -> Self {
        let now = DateTime::now();
        Self {
            id: FormAnswerId::new(uuid::Uuid::new_v4()),
            project_id,
            form_id,
            items,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn new(
        id: FormAnswerId,
        project_id: ProjectId,
        form_id: FormId,
        items: Vec<FormAnswerItem>,
        created_at: DateTime,
        updated_at: DateTime,
    ) -> Self {
        Self {
            id,
            project_id,
            form_id,
            items,
            created_at,
            updated_at,
        }
    }

    pub fn destruct(self) -> DestructedFormAnswer {
        DestructedFormAnswer {
            id: self.id,
            project_id: self.project_id,
            form_id: self.form_id,
            items: self.items,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructedFormAnswer {
    pub id: FormAnswerId,
    pub project_id: ProjectId,
    pub form_id: FormId,
    pub items: Vec<FormAnswerItem>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl FormAnswer {
    pub fn is_updatable_by(&self, actor: &Actor, owned_project_id: Option<ProjectId>) -> bool {
        owned_project_id
            .map(|project_id| self.project_id == project_id)
            .unwrap_or(false)
            || actor.has_permission(Permissions::UPDATE_FORM_ANSWER_ALL)
    }

    pub fn set_items(
        &mut self,
        actor: &Actor,
        owned_project_id: Option<ProjectId>,
        items: Vec<FormAnswerItem>,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor, owned_project_id));
        self.items = items;
        Ok(())
    }

    pub fn list_file_items(&self) -> Vec<(FormItemId, Vec<FileId>)> {
        self.items
            .iter()
            .filter_map(|item| match &item.kind {
                FormAnswerItemKind::File(file) => {
                    Some((item.item_id().clone(), file.clone().value()))
                }
                _ => None,
            })
            .collect()
    }
}

impl_value_object!(FormAnswerId(uuid::Uuid));
#[derive(Debug, Error)]
pub enum FormAnswerIdError {
    #[error("Invalid UUID")]
    InvalidUuid,
}

impl TryFrom<String> for FormAnswerId {
    type Error = FormAnswerIdError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let uuid = uuid::Uuid::from_str(&value).map_err(|_| FormAnswerIdError::InvalidUuid)?;
        Ok(Self(uuid))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct FormAnswerItem {
    #[getset(get = "pub")]
    item_id: FormItemId,
    #[getset(get = "pub")]
    kind: FormAnswerItemKind,
}

impl FormAnswerItem {
    pub fn new(item_id: FormItemId, kind: FormAnswerItemKind) -> Self {
        Self { item_id, kind }
    }

    pub fn destruct(self) -> DestructedFormAnswerItem {
        DestructedFormAnswerItem {
            item_id: self.item_id,
            kind: self.kind,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructedFormAnswerItem {
    pub item_id: FormItemId,
    pub kind: FormAnswerItemKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormAnswerItemKind {
    String(FormAnswerItemString),
    Int(FormAnswerItemInt),
    ChooseOne(FormAnswerItemChooseOne),
    ChooseMany(FormAnswerItemChooseMany),
    File(FormAnswerItemFile),
}

impl_value_object!(FormAnswerItemString(String));
impl_value_object!(FormAnswerItemInt(i32));
impl_value_object!(FormAnswerItemChooseOne(String));
impl_value_object!(FormAnswerItemChooseMany(Vec<String>));
impl_value_object!(FormAnswerItemFile(Vec<FileId>));
