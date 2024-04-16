use std::str::FromStr;

use getset::Getters;
use thiserror::Error;

use crate::entity::file_data::FileId;
use crate::entity::project::{Project, ProjectAttributes, ProjectCategories};
use crate::{ensure, impl_value_object};

use super::actor::Actor;
use super::common::datetime::DateTime;
use super::permission::{PermissionDeniedError, Permissions};

#[derive(Debug, Error)]
pub enum FormError {
    #[error("The end time is earlier than the start time")]
    EndTimeEarlierThanStartTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct Form {
    #[getset(get = "pub")]
    id: FormId,
    #[getset(get = "pub")]
    title: FormTitle,
    #[getset(get = "pub")]
    description: FormDescription,
    #[getset(get = "pub")]
    starts_at: DateTime,
    #[getset(get = "pub")]
    ends_at: DateTime,
    #[getset(get = "pub")]
    categories: ProjectCategories,
    #[getset(get = "pub")]
    attributes: ProjectAttributes,
    #[getset(get = "pub")]
    items: Vec<FormItem>,
    #[getset(get = "pub")]
    attachments: Vec<FileId>,
}

impl Form {
    pub fn create(
        title: FormTitle,
        description: FormDescription,
        starts_at: DateTime,
        ends_at: DateTime,
        categories: ProjectCategories,
        attributes: ProjectAttributes,
        items: Vec<FormItem>,
        attachments: Vec<FileId>,
    ) -> Result<Self, FormError> {
        if starts_at.clone().value() > ends_at.clone().value() {
            return Err(FormError::EndTimeEarlierThanStartTime);
        }

        Ok(Self {
            id: FormId::new(uuid::Uuid::new_v4()),
            title,
            description,
            starts_at,
            ends_at,
            categories,
            attributes,
            items,
            attachments,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: FormId,
        title: FormTitle,
        description: FormDescription,
        starts_at: DateTime,
        ends_at: DateTime,
        categories: ProjectCategories,
        attributes: ProjectAttributes,
        items: Vec<FormItem>,
        attachments: Vec<FileId>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            starts_at,
            ends_at,
            categories,
            attributes,
            items,
            attachments,
        }
    }

    pub fn destruct(self) -> DestructedForm {
        DestructedForm {
            id: self.id,
            title: self.title,
            description: self.description,
            starts_at: self.starts_at,
            ends_at: self.ends_at,
            categories: self.categories,
            attributes: self.attributes,
            items: self.items,
            attachments: self.attachments,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructedForm {
    pub id: FormId,
    pub title: FormTitle,
    pub description: FormDescription,
    pub starts_at: DateTime,
    pub ends_at: DateTime,
    pub categories: ProjectCategories,
    pub attributes: ProjectAttributes,
    pub items: Vec<FormItem>,
    pub attachments: Vec<FileId>,
}

impl Form {
    pub fn is_updatable_by(&self, actor: &Actor) -> bool {
        actor.has_permission(Permissions::UPDATE_FORM_ALL)
    }

    pub fn set_title(
        &mut self,
        actor: &Actor,
        title: FormTitle,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.title = title;
        Ok(())
    }

    pub fn set_description(
        &mut self,
        actor: &Actor,
        description: FormDescription,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.description = description;
        Ok(())
    }

    pub fn set_starts_at(
        &mut self,
        actor: &Actor,
        starts_at: DateTime,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.starts_at = starts_at;
        Ok(())
    }

    pub fn set_ends_at(
        &mut self,
        actor: &Actor,
        ends_at: DateTime,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.ends_at = ends_at;
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

    pub fn set_items(
        &mut self,
        actor: &Actor,
        items: Vec<FormItem>,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.items = items;
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

    // この申請が引数に与えられた企画を対象にしたものであるかを返す
    pub fn is_sent_to(&self, project: &Project) -> bool {
        self.categories.matches(*project.category())
            && self.attributes.matches(*project.attributes())
    }

    pub fn find_item(&self, item_id: &FormItemId) -> Option<&FormItem> {
        self.items.iter().find(|item| item.id() == item_id)
    }
}

impl_value_object!(FormId(uuid::Uuid));
#[derive(Debug, Error)]
pub enum FormIdError {
    #[error("Invalid UUID")]
    InvalidUuid,
}

impl TryFrom<String> for FormId {
    type Error = FormIdError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let uuid = uuid::Uuid::from_str(&value).map_err(|_| FormIdError::InvalidUuid)?;
        Ok(Self(uuid))
    }
}

impl_value_object!(FormTitle(String));
impl_value_object!(FormDescription(String));

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct FormItem {
    #[getset(get = "pub")]
    id: FormItemId,
    #[getset(get = "pub")]
    name: FormItemName,
    #[getset(get = "pub")]
    description: FormItemDescription,
    #[getset(get = "pub")]
    required: FormItemRequired,
    #[getset(get = "pub")]
    kind: FormItemKind,
}

impl FormItem {
    pub fn create(
        name: FormItemName,
        description: FormItemDescription,
        required: FormItemRequired,
        kind: FormItemKind,
    ) -> Self {
        Self {
            id: FormItemId::new(uuid::Uuid::new_v4()),
            name,
            description,
            required,
            kind,
        }
    }

    pub fn new(
        id: FormItemId,
        name: FormItemName,
        description: FormItemDescription,
        required: FormItemRequired,
        kind: FormItemKind,
    ) -> Self {
        Self {
            id,
            name,
            description,
            required,
            kind,
        }
    }

    pub fn destruct(self) -> DestructedFormItem {
        DestructedFormItem {
            id: self.id,
            name: self.name,
            description: self.description,
            required: self.required,
            kind: self.kind,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct DestructedFormItem {
    pub id: FormItemId,
    pub name: FormItemName,
    pub description: FormItemDescription,
    pub required: FormItemRequired,
    pub kind: FormItemKind,
}

impl_value_object!(FormItemId(uuid::Uuid));
#[derive(Debug, Error)]
pub enum FormItemIdError {
    #[error("Invalid UUID")]
    InvalidUuid,
}

impl TryFrom<String> for FormItemId {
    type Error = FormItemIdError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let uuid = uuid::Uuid::from_str(&value).map_err(|_| FormItemIdError::InvalidUuid)?;
        Ok(Self(uuid))
    }
}

impl_value_object!(FormItemName(String));
impl_value_object!(FormItemDescription(String));
impl_value_object!(FormItemRequired(bool));

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormItemKind {
    String(FormItemString),
    Int(FormItemInt),
    ChooseOne(FormItemChooseOne),
    ChooseMany(FormItemChooseMany),
    File(FormItemFile),
}

impl FormItemKind {
    pub fn new_string(
        min_length: Option<FormItemMinLength>,
        max_length: Option<FormItemMaxLength>,
        allow_newline: FormItemAllowNewline,
    ) -> Self {
        Self::String(FormItemString {
            min_length,
            max_length,
            allow_newline,
        })
    }

    pub fn new_int(min: Option<FormItemMin>, max: Option<FormItemMax>) -> Self {
        Self::Int(FormItemInt { min, max })
    }

    pub fn new_choose_one(options: Vec<FormItemOption>) -> Self {
        Self::ChooseOne(FormItemChooseOne { options })
    }

    pub fn new_choose_many(
        options: Vec<FormItemOption>,
        min_selection: Option<FormItemMinSelection>,
        max_selection: Option<FormItemMaxSelection>,
    ) -> Self {
        Self::ChooseMany(FormItemChooseMany {
            options,
            min_selection,
            max_selection,
        })
    }

    pub fn new_file(
        extensions: Option<Vec<FormItemExtension>>,
        limit: Option<FormItemLimit>,
    ) -> Self {
        Self::File(FormItemFile { extensions, limit })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct FormItemString {
    #[getset(get = "pub")]
    min_length: Option<FormItemMinLength>,
    #[getset(get = "pub")]
    max_length: Option<FormItemMaxLength>,
    #[getset(get = "pub")]
    allow_newline: FormItemAllowNewline,
}

impl FormItemString {
    pub fn destruct(self) -> DestructedFormItemString {
        DestructedFormItemString {
            min_length: self.min_length,
            max_length: self.max_length,
            allow_newline: self.allow_newline,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct DestructedFormItemString {
    pub min_length: Option<FormItemMinLength>,
    pub max_length: Option<FormItemMaxLength>,
    pub allow_newline: FormItemAllowNewline,
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct FormItemInt {
    #[getset(get = "pub")]
    min: Option<FormItemMin>,
    #[getset(get = "pub")]
    max: Option<FormItemMax>,
}

impl FormItemInt {
    pub fn destruct(self) -> DestructedFormItemInt {
        DestructedFormItemInt {
            min: self.min,
            max: self.max,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct DestructedFormItemInt {
    pub min: Option<FormItemMin>,
    pub max: Option<FormItemMax>,
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct FormItemChooseOne {
    #[getset(get = "pub")]
    options: Vec<FormItemOption>,
}

impl FormItemChooseOne {
    pub fn destruct(self) -> DestructedFormItemChooseOne {
        DestructedFormItemChooseOne {
            options: self.options,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct DestructedFormItemChooseOne {
    pub options: Vec<FormItemOption>,
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct FormItemChooseMany {
    #[getset(get = "pub")]
    options: Vec<FormItemOption>,
    #[getset(get = "pub")]
    min_selection: Option<FormItemMinSelection>,
    #[getset(get = "pub")]
    max_selection: Option<FormItemMaxSelection>,
}

impl FormItemChooseMany {
    pub fn destruct(self) -> DestructedFormItemChooseMany {
        DestructedFormItemChooseMany {
            options: self.options,
            min_selection: self.min_selection,
            max_selection: self.max_selection,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct DestructedFormItemChooseMany {
    pub options: Vec<FormItemOption>,
    pub min_selection: Option<FormItemMinSelection>,
    pub max_selection: Option<FormItemMaxSelection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct FormItemFile {
    #[getset(get = "pub")]
    extensions: Option<Vec<FormItemExtension>>,
    #[getset(get = "pub")]
    limit: Option<FormItemLimit>,
}

impl FormItemFile {
    pub fn destruct(self) -> DestructedFormItemFile {
        DestructedFormItemFile {
            extensions: self.extensions,
            limit: self.limit,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct DestructedFormItemFile {
    pub extensions: Option<Vec<FormItemExtension>>,
    pub limit: Option<FormItemLimit>,
}

impl_value_object!(FormItemMinLength(i32));
impl_value_object!(FormItemMaxLength(i32));
impl_value_object!(FormItemAllowNewline(bool));
impl_value_object!(FormItemMin(i32));
impl_value_object!(FormItemMax(i32));
impl_value_object!(FormItemOption(String));
impl_value_object!(FormItemMinSelection(i32));
impl_value_object!(FormItemMaxSelection(i32));
impl_value_object!(FormItemExtension(String));
impl_value_object!(FormItemLimit(i32));

#[cfg(test)]
mod tests {
    use crate::{entity::form::FormError, test::fixture};

    use super::Form;

    #[test]
    fn 申請の開始時間が終了時間より後ならばエラーを返す() {
        let form = Form::create(
            fixture::form::title1(),
            fixture::form::description1(),
            fixture::form::ends_at1(),
            fixture::form::starts_at1(),
            fixture::form::categories1(),
            fixture::form::attributes1(),
            fixture::form::items1(),
            fixture::form::attachments1(),
        );
        assert!(matches!(form, Err(FormError::EndTimeEarlierThanStartTime)));
    }
}
