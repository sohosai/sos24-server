use crate::{
    entity::{
        form_answer::{
            FormAnswer, FormAnswerId, FormAnswerItem, FormAnswerItemKind, FormAnswerItemString,
        },
        project::ProjectId,
    },
    test::fixture::form,
};

pub fn id1() -> FormAnswerId {
    FormAnswerId::new(uuid::Uuid::from_u128(1))
}

pub fn items1() -> Vec<FormAnswerItem> {
    vec![FormAnswerItem::new(
        form::formitem_id1(),
        FormAnswerItemKind::String(FormAnswerItemString::new("あ".to_string())),
    )]
}

pub fn form_answer1(project_id: ProjectId) -> FormAnswer {
    FormAnswer::new(id1(), project_id, form::id1(), items1())
}

pub fn id2() -> FormAnswerId {
    FormAnswerId::new(uuid::Uuid::from_u128(1))
}

pub fn items2() -> Vec<FormAnswerItem> {
    vec![FormAnswerItem::new(
        form::formitem_id1(),
        FormAnswerItemKind::String(FormAnswerItemString::new("い".to_string())),
    )]
}

pub fn form_answer2(project_id: ProjectId) -> FormAnswer {
    FormAnswer::new(id1(), project_id, form::id1(), items1())
}
