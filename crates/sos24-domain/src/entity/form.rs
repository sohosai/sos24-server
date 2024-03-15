use std::str::FromStr;

use getset::Getters;
use thiserror::Error;

use crate::impl_value_object;

use super::common::datetime::DateTime;

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
    items: Vec<FormItem>,
}

impl Form {
    pub fn create(
        title: FormTitle,
        description: FormDescription,
        starts_at: DateTime,
        ends_at: DateTime,
        items: Vec<FormItem>,
    ) -> Self {
        Self {
            id: FormId::new(uuid::Uuid::new_v4()),
            title,
            description,
            starts_at,
            ends_at,
            items,
        }
    }

    pub fn new(
        id: FormId,
        title: FormTitle,
        description: FormDescription,
        starts_at: DateTime,
        ends_at: DateTime,
        items: Vec<FormItem>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            starts_at,
            ends_at,
            items,
        }
    }

    pub fn destruct(self) -> DestructedForm {
        DestructedForm {
            id: self.id,
            title: self.title,
            description: self.description,
            starts_at: self.starts_at,
            ends_at: self.ends_at,
            items: self.items,
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
    pub items: Vec<FormItem>,
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
    name: FormItemName,
    #[getset(get = "pub")]
    description: FormItemDescription,
    #[getset(get = "pub")]
    required: FormItemRequired,
    #[getset(get = "pub")]
    kind: FormItemKind,
}

impl FormItem {
    pub fn new(
        name: FormItemName,
        description: FormItemDescription,
        required: FormItemRequired,
        kind: FormItemKind,
    ) -> Self {
        Self {
            name,
            description,
            required,
            kind,
        }
    }

    pub fn destruct(self) -> DestructedFormItem {
        DestructedFormItem {
            name: self.name,
            description: self.description,
            required: self.required,
            kind: self.kind,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct DestructedFormItem {
    pub name: FormItemName,
    pub description: FormItemDescription,
    pub required: FormItemRequired,
    pub kind: FormItemKind,
}

impl_value_object!(FormItemName(String));
impl_value_object!(FormItemDescription(String));
impl_value_object!(FormItemRequired(bool));

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormItemKind {
    String {
        min_length: FormItemMinLength,
        max_length: FormItemMaxLength,
        allow_newline: FormItemAllowNewline,
    },
    Int {
        min: FormItemMin,
        max: FormItemMax,
    },
    ChooseOne {
        options: Vec<FormItemOption>,
    },
    ChooseMany {
        options: Vec<FormItemOption>,
        min_selection: FormItemMinSelection,
        max_selection: FormItemMaxSelection,
    },
    File {
        extentions: Vec<FormItemExtention>,
        limit: FormItemLimit,
    },
}

impl_value_object!(FormItemMinLength(i32));
impl_value_object!(FormItemMaxLength(i32));
impl_value_object!(FormItemAllowNewline(bool));
impl_value_object!(FormItemMin(i32));
impl_value_object!(FormItemMax(i32));
impl_value_object!(FormItemOption(String));
impl_value_object!(FormItemMinSelection(i32));
impl_value_object!(FormItemMaxSelection(i32));
impl_value_object!(FormItemExtention(String));
impl_value_object!(FormItemLimit(i32));
