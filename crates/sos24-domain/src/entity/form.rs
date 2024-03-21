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
    String(FormItemString),
    Int(FormItemInt),
    ChooseOne(FormItemChooseOne),
    ChooseMany(FormItemChooseMany),
    File(FormItemFile),
}

impl FormItemKind {
    pub fn new_string(
        min_length: FormItemMinLength,
        max_length: FormItemMaxLength,
        allow_newline: FormItemAllowNewline,
    ) -> Self {
        Self::String(FormItemString {
            min_length,
            max_length,
            allow_newline,
        })
    }

    pub fn new_int(min: FormItemMin, max: FormItemMax) -> Self {
        Self::Int(FormItemInt { min, max })
    }

    pub fn new_choose_one(options: Vec<FormItemOption>) -> Self {
        Self::ChooseOne(FormItemChooseOne { options })
    }

    pub fn new_choose_many(
        options: Vec<FormItemOption>,
        min_selection: FormItemMinSelection,
        max_selection: FormItemMaxSelection,
    ) -> Self {
        Self::ChooseMany(FormItemChooseMany {
            options,
            min_selection,
            max_selection,
        })
    }

    pub fn new_file(extentions: Vec<FormItemExtention>, limit: FormItemLimit) -> Self {
        Self::File(FormItemFile { extentions, limit })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct FormItemString {
    #[getset(get = "pub")]
    min_length: FormItemMinLength,
    #[getset(get = "pub")]
    max_length: FormItemMaxLength,
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
    pub min_length: FormItemMinLength,
    pub max_length: FormItemMaxLength,
    pub allow_newline: FormItemAllowNewline,
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct FormItemInt {
    #[getset(get = "pub")]
    min: FormItemMin,
    #[getset(get = "pub")]
    max: FormItemMax,
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
    pub min: FormItemMin,
    pub max: FormItemMax,
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
    min_selection: FormItemMinSelection,
    #[getset(get = "pub")]
    max_selection: FormItemMaxSelection,
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
    pub min_selection: FormItemMinSelection,
    pub max_selection: FormItemMaxSelection,
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct FormItemFile {
    #[getset(get = "pub")]
    extentions: Vec<FormItemExtention>,
    #[getset(get = "pub")]
    limit: FormItemLimit,
}

impl FormItemFile {
    pub fn destruct(self) -> DestructedFormItemFile {
        DestructedFormItemFile {
            extentions: self.extentions,
            limit: self.limit,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct DestructedFormItemFile {
    pub extentions: Vec<FormItemExtention>,
    pub limit: FormItemLimit,
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
