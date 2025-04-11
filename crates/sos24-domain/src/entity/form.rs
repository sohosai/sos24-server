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
    #[error("The minimum length is greater than the maximum length")]
    MinLengthGreaterThanMaxLength,
    #[error("The minimum value is greater than the maximum value")]
    MinGreaterThanMax,
    #[error("The options is empty")]
    EmptyOptions,
    #[error("The minimum selection is greater than the options")]
    MinSelectionGreaterThanOptions,
    #[error("The minimum selection is greater than the maximum selection")]
    MinSelectionGreaterThanMaxSelection,
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct Form {
    #[getset(get = "pub")]
    id: FormId,
    #[getset(get = "pub")]
    state: FormState,
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
    is_notified: FormIsNotified,
    #[getset(get = "pub")]
    items: Vec<FormItem>,
    #[getset(get = "pub")]
    attachments: Vec<FileId>,
    #[getset(get = "pub")]
    created_at: DateTime,
    #[getset(get = "pub")]
    updated_at: DateTime,
}

impl Form {
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        state: FormState,
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

        let now = DateTime::now();
        Ok(Self {
            id: FormId::new(uuid::Uuid::new_v4()),
            state,
            title,
            description,
            starts_at,
            ends_at,
            categories,
            attributes,
            is_notified: FormIsNotified::new(false),
            items,
            attachments,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: FormId,
        state: FormState,
        title: FormTitle,
        description: FormDescription,
        starts_at: DateTime,
        ends_at: DateTime,
        categories: ProjectCategories,
        attributes: ProjectAttributes,
        is_notified: FormIsNotified,
        items: Vec<FormItem>,
        attachments: Vec<FileId>,
        created_at: DateTime,
        updated_at: DateTime,
    ) -> Self {
        Self {
            id,
            state,
            title,
            description,
            starts_at,
            ends_at,
            categories,
            attributes,
            is_notified,
            items,
            attachments,
            created_at,
            updated_at,
        }
    }

    pub fn destruct(self) -> DestructedForm {
        DestructedForm {
            id: self.id,
            state: self.state,
            title: self.title,
            description: self.description,
            starts_at: self.starts_at,
            ends_at: self.ends_at,
            categories: self.categories,
            attributes: self.attributes,
            is_notified: self.is_notified,
            items: self.items,
            attachments: self.attachments,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructedForm {
    pub id: FormId,
    pub state: FormState,
    pub title: FormTitle,
    pub description: FormDescription,
    pub starts_at: DateTime,
    pub ends_at: DateTime,
    pub categories: ProjectCategories,
    pub attributes: ProjectAttributes,
    pub is_notified: FormIsNotified,
    pub items: Vec<FormItem>,
    pub attachments: Vec<FileId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
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

    pub fn set_notified(&mut self) -> Result<(), PermissionDeniedError> {
        self.is_notified = FormIsNotified::new(true);
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

    pub fn is_started(&self, now: &chrono::DateTime<chrono::Utc>) -> bool {
        &self.starts_at.clone().value() <= now
    }

    pub fn is_ended(&self, now: &chrono::DateTime<chrono::Utc>) -> bool {
        &self.ends_at.clone().value() <= now
    }

    pub fn can_be_updated(&self, actor: &Actor, now: &chrono::DateTime<chrono::Utc>) -> bool {
        !self.is_ended(now) || actor.has_permission(Permissions::UPDATE_FORM_ANSWER_ANYTIME)
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormState {
    Draft,
    Scheduled,
    Published,
}

impl_value_object!(FormTitle(String));
impl_value_object!(FormDescription(String));
impl_value_object!(FormIsNotified(bool));

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct FormItem {
    #[getset(get = "pub")]
    id: FormItemId,
    #[getset(get = "pub")]
    name: FormItemName,
    #[getset(get = "pub")]
    description: Option<FormItemDescription>,
    #[getset(get = "pub")]
    required: FormItemRequired,
    #[getset(get = "pub")]
    kind: FormItemKind,
}

impl FormItem {
    pub fn create(
        name: FormItemName,
        description: Option<FormItemDescription>,
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
        description: Option<FormItemDescription>,
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
    pub description: Option<FormItemDescription>,
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
    ) -> Result<Self, FormError> {
        if let (Some(min_length), Some(max_length)) = (min_length.clone(), max_length.clone()) {
            if min_length.value() > max_length.value() {
                return Err(FormError::MinLengthGreaterThanMaxLength);
            }
        }

        Ok(Self::String(FormItemString {
            min_length,
            max_length,
            allow_newline,
        }))
    }

    pub fn new_int(min: Option<FormItemMin>, max: Option<FormItemMax>) -> Result<Self, FormError> {
        if let (Some(min), Some(max)) = (min.clone(), max.clone()) {
            if min.value() > max.value() {
                return Err(FormError::MinGreaterThanMax);
            }
        }

        Ok(Self::Int(FormItemInt { min, max }))
    }

    pub fn new_choose_one(options: Vec<FormItemOption>) -> Result<Self, FormError> {
        if options.is_empty() {
            return Err(FormError::EmptyOptions);
        }

        Ok(Self::ChooseOne(FormItemChooseOne { options }))
    }

    pub fn new_choose_many(
        options: Vec<FormItemOption>,
        min_selection: Option<FormItemMinSelection>,
        max_selection: Option<FormItemMaxSelection>,
    ) -> Result<Self, FormError> {
        if options.is_empty() {
            return Err(FormError::EmptyOptions);
        }
        if let Some(min_selection) = min_selection.clone() {
            if min_selection.value() > options.len() as u32 {
                return Err(FormError::MinSelectionGreaterThanOptions);
            }
        }
        if let (Some(min_selection), Some(max_selection)) =
            (min_selection.clone(), max_selection.clone())
        {
            if min_selection.value() > max_selection.value() {
                return Err(FormError::MinSelectionGreaterThanMaxSelection);
            }
        }

        Ok(Self::ChooseMany(FormItemChooseMany {
            options,
            min_selection,
            max_selection,
        }))
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

impl_value_object!(FormItemMinLength(u32));
impl_value_object!(FormItemMaxLength(u32));
impl_value_object!(FormItemAllowNewline(bool));
impl_value_object!(FormItemMin(i32));
impl_value_object!(FormItemMax(i32));
impl_value_object!(FormItemOption(String));
impl_value_object!(FormItemMinSelection(u32));
impl_value_object!(FormItemMaxSelection(u32));
impl_value_object!(FormItemExtension(String));
impl_value_object!(FormItemLimit(u32));

#[cfg(test)]
mod tests {
    use crate::{
        entity::form::{
            FormError, FormItemAllowNewline, FormItemKind, FormItemMax, FormItemMaxLength,
            FormItemMaxSelection, FormItemMin, FormItemMinLength, FormItemMinSelection,
            FormItemOption,
        },
        test::fixture,
    };

    use super::Form;

    #[test]
    fn 申請の開始時間が終了時間より前ならばエラーを返さない() {
        let form = Form::create(
            fixture::form::state1(),
            fixture::form::title1(),
            fixture::form::description1(),
            fixture::form::starts_at1_opened(),
            fixture::form::ends_at1_opened(),
            fixture::form::categories1(),
            fixture::form::attributes1(),
            fixture::form::items1(),
            fixture::form::attachments1(),
        );
        assert!(form.is_ok());
    }

    #[test]
    fn 申請の開始時間が終了時間より後ならばエラーを返す() {
        let form = Form::create(
            fixture::form::state1(),
            fixture::form::title1(),
            fixture::form::description1(),
            fixture::form::ends_at1_opened(),
            fixture::form::starts_at1_opened(),
            fixture::form::categories1(),
            fixture::form::attributes1(),
            fixture::form::items1(),
            fixture::form::attachments1(),
        );
        assert!(matches!(form, Err(FormError::EndTimeEarlierThanStartTime)));
    }

    #[test]
    fn 文字列項目の最小文字数が最大文字数以下ならばエラーを返さない() {
        let item = FormItemKind::new_string(
            Some(FormItemMinLength::new(1)),
            Some(FormItemMaxLength::new(2)),
            FormItemAllowNewline::new(false),
        );
        assert!(item.is_ok());
    }

    #[test]
    fn 文字列項目の最小文字数が最大文字数より大きいならばエラーを返す() {
        let item = FormItemKind::new_string(
            Some(FormItemMinLength::new(2)),
            Some(FormItemMaxLength::new(1)),
            FormItemAllowNewline::new(false),
        );
        assert!(matches!(
            item,
            Err(FormError::MinLengthGreaterThanMaxLength)
        ));
    }

    #[test]
    fn 数値項目の最小値が最大値以下ならばエラーを返さない() {
        let item = FormItemKind::new_int(Some(FormItemMin::new(1)), Some(FormItemMax::new(2)));
        assert!(item.is_ok());
    }

    #[test]
    fn 数値項目の最小値が最大値より大きいならばエラーを返す() {
        let item = FormItemKind::new_int(Some(FormItemMin::new(2)), Some(FormItemMax::new(1)));
        assert!(matches!(item, Err(FormError::MinGreaterThanMax)));
    }

    #[test]
    fn 選択肢項目の選択肢が1個以上ならばエラーを返さない() {
        let item = FormItemKind::new_choose_one(vec![FormItemOption::new("a".to_string())]);
        assert!(item.is_ok());
    }

    #[test]
    fn 選択肢項目の選択肢が0個ならばエラーを返す() {
        let item = FormItemKind::new_choose_one(vec![]);
        assert!(matches!(item, Err(FormError::EmptyOptions)));
    }

    #[test]
    fn 複数選択項目の最小選択数が選択肢数以下ならばエラーを返さない() {
        let item = FormItemKind::new_choose_many(
            vec![
                FormItemOption::new("a".to_string()),
                FormItemOption::new("b".to_string()),
            ],
            Some(FormItemMinSelection::new(2)),
            None,
        );
        assert!(item.is_ok());
    }

    #[test]
    fn 複数選択項目の最小選択数が選択肢数より大きいならばエラーを返す() {
        let item = FormItemKind::new_choose_many(
            vec![
                FormItemOption::new("a".to_string()),
                FormItemOption::new("b".to_string()),
            ],
            Some(FormItemMinSelection::new(3)),
            None,
        );
        assert!(matches!(
            item,
            Err(FormError::MinSelectionGreaterThanOptions)
        ));
    }

    #[test]
    fn 複数選択項目の最小選択数が最大選択数以下ならばエラーを返さない() {
        let item = FormItemKind::new_choose_many(
            vec![
                FormItemOption::new("a".to_string()),
                FormItemOption::new("b".to_string()),
            ],
            Some(FormItemMinSelection::new(1)),
            Some(FormItemMaxSelection::new(2)),
        );
        assert!(item.is_ok());
    }

    #[test]
    fn 複数選択項目の最小選択数が最大選択数より大きいならばエラーを返す() {
        let item = FormItemKind::new_choose_many(
            vec![
                FormItemOption::new("a".to_string()),
                FormItemOption::new("b".to_string()),
            ],
            Some(FormItemMinSelection::new(2)),
            Some(FormItemMaxSelection::new(1)),
        );
        assert!(matches!(
            item,
            Err(FormError::MinSelectionGreaterThanMaxSelection)
        ));
    }
}
