use crate::entity::{
    common::datetime::DateTime,
    form::{
        Form, FormDescription, FormId, FormItem, FormItemAllowNewline, FormItemDescription,
        FormItemId, FormItemKind, FormItemMax, FormItemMaxLength, FormItemMin, FormItemMinLength,
        FormItemName, FormItemRequired, FormTitle,
    },
    project::{ProjectAttributes, ProjectCategories},
};

pub fn id1() -> FormId {
    FormId::new(uuid::Uuid::from_u128(1))
}

pub fn title1() -> FormTitle {
    FormTitle::new("そぽたん申請".to_string())
}

pub fn description1() -> FormDescription {
    FormDescription::new("そぽたん申請です".to_string())
}

pub fn starts_at1() -> DateTime {
    DateTime::try_from("2021-01-01T00:00:00+00:00".to_string()).unwrap()
}

pub fn ends_at1() -> DateTime {
    DateTime::try_from("2021-01-01T23:59:59+00:00".to_string()).unwrap()
}

pub fn categories1() -> ProjectCategories {
    ProjectCategories::GENERAL
}

pub fn attributes1() -> ProjectAttributes {
    ProjectAttributes::ACADEMIC
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
}

pub fn items1() -> Vec<FormItem> {
    vec![FormItem::new(
        formitem_id1(),
        formitem_name1(),
        formitem_description1(),
        formitem_required1(),
        formitem_kind1(),
    )]
}

pub fn form1() -> Form {
    Form::new(
        id1(),
        title1(),
        description1(),
        starts_at1(),
        ends_at1(),
        categories1(),
        attributes1(),
        items1(),
    )
}

pub fn id2() -> FormId {
    FormId::new(uuid::Uuid::from_u128(2))
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
    FormItemKind::new_int(Some(FormItemMin::new(1)), Some(FormItemMax::new(2)))
}

pub fn items2() -> Vec<FormItem> {
    vec![FormItem::new(
        formitem_id2(),
        formitem_name2(),
        formitem_description2(),
        formitem_required2(),
        formitem_kind2(),
    )]
}

pub fn form2() -> Form {
    Form::new(
        id2(),
        title2(),
        description2(),
        starts_at2(),
        ends_at2(),
        categories2(),
        attributes2(),
        items2(),
    )
}