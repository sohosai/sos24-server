use crate::entity::file_data::FileId;
use crate::entity::form::FormIsNotified;
use crate::entity::{
    common::datetime::DateTime,
    form::{
        Form, FormDescription, FormId, FormItem, FormItemAllowNewline, FormItemDescription,
        FormItemId, FormItemKind, FormItemMax, FormItemMaxLength, FormItemMin, FormItemMinLength,
        FormItemName, FormItemRequired, FormState, FormTitle,
    },
    project::{ProjectAttributes, ProjectCategories},
};

use super::datetime;

pub fn id1() -> FormId {
    FormId::new(uuid::Uuid::from_u128(1))
}

pub fn state1() -> FormState {
    FormState::Draft
}

pub fn title1() -> FormTitle {
    FormTitle::new("そぽたん申請".to_string())
}

pub fn description1() -> FormDescription {
    FormDescription::new("そぽたん申請です".to_string())
}

pub fn starts_at1_opened() -> DateTime {
    DateTime::new(
        chrono::Utc::now()
            .checked_sub_days(chrono::Days::new(1))
            .unwrap(),
    )
}

pub fn ends_at1_opened() -> DateTime {
    DateTime::new(
        chrono::Utc::now()
            .checked_add_days(chrono::Days::new(1))
            .unwrap(),
    )
}

pub fn starts_at1_closed() -> DateTime {
    DateTime::new(
        chrono::Utc::now()
            .checked_sub_days(chrono::Days::new(2))
            .unwrap(),
    )
}

pub fn ends_at1_closed() -> DateTime {
    DateTime::new(
        chrono::Utc::now()
            .checked_sub_days(chrono::Days::new(1))
            .unwrap(),
    )
}

pub fn categories1() -> ProjectCategories {
    ProjectCategories::GENERAL
}

pub fn attributes1() -> ProjectAttributes {
    ProjectAttributes::ACADEMIC
}

pub fn is_notified1() -> FormIsNotified {
    FormIsNotified::new(false)
}

pub fn formitem_id1() -> FormItemId {
    FormItemId::new(uuid::Uuid::from_u128(1))
}

pub fn formitem_name1() -> FormItemName {
    FormItemName::new("文字列".to_string())
}

pub fn formitem_description1() -> FormItemDescription {
    FormItemDescription::new("文字列です".to_string())
}

pub fn formitem_required1() -> FormItemRequired {
    FormItemRequired::new(true)
}

pub fn formitem_kind1() -> FormItemKind {
    FormItemKind::new_string(
        Some(FormItemMinLength::new(1)),
        Some(FormItemMaxLength::new(10)),
        FormItemAllowNewline::new(true),
    )
    .unwrap()
}

pub fn items1() -> Vec<FormItem> {
    vec![FormItem::new(
        formitem_id1(),
        formitem_name1(),
        Some(formitem_description1()),
        formitem_required1(),
        formitem_kind1(),
    )]
}

pub fn attachments1() -> Vec<FileId> {
    vec![]
}

pub fn form1_opened() -> Form {
    Form::new(
        id1(),
        state1(),
        title1(),
        description1(),
        starts_at1_opened(),
        ends_at1_opened(),
        categories1(),
        attributes1(),
        is_notified1(),
        items1(),
        attachments1(),
        datetime::now(),
        datetime::now(),
    )
}

pub fn form1_closed() -> Form {
    Form::new(
        id1(),
        state1(),
        title1(),
        description1(),
        starts_at1_closed(),
        ends_at1_closed(),
        categories1(),
        attributes1(),
        is_notified1(),
        items1(),
        attachments1(),
        datetime::now(),
        datetime::now(),
    )
}

pub fn id2() -> FormId {
    FormId::new(uuid::Uuid::from_u128(2))
}

pub fn state2() -> FormState {
    FormState::Published
}

pub fn title2() -> FormTitle {
    FormTitle::new("んぽたそ申請".to_string())
}

pub fn description2() -> FormDescription {
    FormDescription::new("んぽたそ申請です".to_string())
}

pub fn starts_at2() -> DateTime {
    DateTime::try_from("2021-01-02T00:00:00+00:00".to_string()).unwrap()
}

pub fn ends_at2() -> DateTime {
    DateTime::try_from("2021-01-02T23:59:59+00:00".to_string()).unwrap()
}

pub fn categories2() -> ProjectCategories {
    ProjectCategories::STAGE_1A
}

pub fn attributes2() -> ProjectAttributes {
    ProjectAttributes::OUTSIDE
}

pub fn is_notified2() -> FormIsNotified {
    FormIsNotified::new(false)
}

pub fn formitem_id2() -> FormItemId {
    FormItemId::new(uuid::Uuid::from_u128(2))
}

pub fn formitem_name2() -> FormItemName {
    FormItemName::new("数字".to_string())
}

pub fn formitem_description2() -> FormItemDescription {
    FormItemDescription::new("数字です".to_string())
}

pub fn formitem_required2() -> FormItemRequired {
    FormItemRequired::new(true)
}

pub fn formitem_kind2() -> FormItemKind {
    FormItemKind::new_int(Some(FormItemMin::new(1)), Some(FormItemMax::new(2))).unwrap()
}

pub fn items2() -> Vec<FormItem> {
    vec![FormItem::new(
        formitem_id2(),
        formitem_name2(),
        Some(formitem_description2()),
        formitem_required2(),
        formitem_kind2(),
    )]
}

pub fn attachments2() -> Vec<FileId> {
    vec![]
}

pub fn form2() -> Form {
    Form::new(
        id2(),
        state2(),
        title2(),
        description2(),
        starts_at2(),
        ends_at2(),
        categories2(),
        attributes2(),
        is_notified1(),
        items2(),
        attachments2(),
        datetime::now(),
        datetime::now(),
    )
}
