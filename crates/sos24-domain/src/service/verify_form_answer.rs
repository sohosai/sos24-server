use thiserror::Error;
use unicode_segmentation::UnicodeSegmentation;

use crate::entity::{
    form::{
        Form, FormItem, FormItemChooseMany, FormItemChooseOne, FormItemFile, FormItemId,
        FormItemInt, FormItemKind, FormItemOption, FormItemString,
    },
    form_answer::{
        FormAnswer, FormAnswerItem, FormAnswerItemChooseMany, FormAnswerItemChooseOne,
        FormAnswerItemFile, FormAnswerItemInt, FormAnswerItemKind, FormAnswerItemString,
    },
};

#[derive(Debug, Error)]
pub enum VerifyFormAnswerError {
    #[error("Answer item {0:?} is missing")]
    MissingAnswerItem(FormItemId),
    #[error("Answer item {0:?} has invalid kind")]
    InvalidAnswerItemKind(FormItemId),
    #[error("String answer item {0:?} is too short (min: {1})")]
    TooShortString(FormItemId, i32),
    #[error("String answer item {0:?} is too long (max: {1})")]
    TooLongString(FormItemId, i32),
    #[error("String answer item {0:?} contains newline")]
    NewlineNotAllowed(FormItemId),
    #[error("Int answer item {0:?} is too small (min: {1})")]
    TooSmallInt(FormItemId, i32),
    #[error("Int answer item {0:?} is too large (max: {1})")]
    TooLargeInt(FormItemId, i32),
    #[error("ChooseOne answer item {0:?} has invalid option: {1}")]
    InvalidChooseOneOption(FormItemId, String),
    #[error("ChooseMany answer item {0:?} has invalid option: {1}")]
    InvalidChooseManyOption(FormItemId, String),
    #[error("ChooseOne answer item {0:?} has too few options (min: {1})")]
    TooFewOptionsChooseMany(FormItemId, i32),
    #[error("ChooseOne answer item {0:?} has too many options (max: {1})")]
    TooManyOptionsChooseMany(FormItemId, i32),
    #[error("File answer item {0:?} has too many files (max: {1})")]
    TooManyFiles(FormItemId, i32),
}

pub fn verify(form: &Form, answer: &FormAnswer) -> Result<(), VerifyFormAnswerError> {
    for form_item in form.items() {
        let answer_item = answer
            .items()
            .iter()
            .find(|answer_item| answer_item.item_id() == form_item.id());

        let is_required = form_item.required().clone().value();
        if is_required && answer_item.is_none() {
            return Err(VerifyFormAnswerError::MissingAnswerItem(
                form_item.id().clone(),
            ));
        }

        match answer_item {
            Some(answer_item) => verify_item(form_item, answer_item)?,
            None => continue,
        }
    }

    Ok(())
}

fn verify_item(
    form_item: &FormItem,
    answer_item: &FormAnswerItem,
) -> Result<(), VerifyFormAnswerError> {
    let item_id = form_item.id().clone();
    match (form_item.kind().clone(), answer_item.kind().clone()) {
        (FormItemKind::String(form_item), FormAnswerItemKind::String(answer_item)) => {
            verify_item_string(item_id, form_item, answer_item)
        }
        (FormItemKind::Int(form_item), FormAnswerItemKind::Int(answer_item)) => {
            verify_item_int(item_id, form_item, answer_item)
        }
        (FormItemKind::ChooseOne(form_item), FormAnswerItemKind::ChooseOne(answer_item)) => {
            verify_item_choose_one(item_id, form_item, answer_item)
        }
        (FormItemKind::ChooseMany(form_item), FormAnswerItemKind::ChooseMany(answer_item)) => {
            verify_item_choose_many(item_id, form_item, answer_item)
        }
        (FormItemKind::File(form_item), FormAnswerItemKind::File(answer_item)) => {
            verify_item_file(item_id, form_item, answer_item)
        }
        _ => Err(VerifyFormAnswerError::InvalidAnswerItemKind(
            form_item.id().clone(),
        )),
    }
}

fn verify_item_string(
    item_id: FormItemId,
    form_string: FormItemString,
    answer_string: FormAnswerItemString,
) -> Result<(), VerifyFormAnswerError> {
    let value = answer_string.value();
    let value_len = value.graphemes(true).count();

    if let Some(min_length) = form_string.min_length().clone() {
        let min_length = min_length.value();
        if value_len < min_length as usize {
            return Err(VerifyFormAnswerError::TooShortString(item_id, min_length));
        }
    }

    if let Some(max_length) = form_string.max_length().clone() {
        let max_length = max_length.value();
        if value_len > max_length as usize {
            return Err(VerifyFormAnswerError::TooLongString(item_id, max_length));
        }
    }

    if !form_string.allow_newline().clone().value() && value.contains('\n') {
        return Err(VerifyFormAnswerError::NewlineNotAllowed(item_id));
    }

    Ok(())
}

fn verify_item_int(
    item_id: FormItemId,
    form_int: FormItemInt,
    answer_int: FormAnswerItemInt,
) -> Result<(), VerifyFormAnswerError> {
    let value = answer_int.value();

    if let Some(min) = form_int.min() {
        let min = min.clone().value();
        if value < min {
            return Err(VerifyFormAnswerError::TooSmallInt(item_id, min));
        }
    }

    if let Some(max) = form_int.max() {
        let max = max.clone().value();
        if value > max {
            return Err(VerifyFormAnswerError::TooLargeInt(item_id, max));
        }
    }

    Ok(())
}

fn verify_item_choose_one(
    item_id: FormItemId,
    form_choose_one: FormItemChooseOne,
    answer_choose_one: FormAnswerItemChooseOne,
) -> Result<(), VerifyFormAnswerError> {
    let value = FormItemOption::new(answer_choose_one.value());

    if !form_choose_one.options().contains(&value) {
        return Err(VerifyFormAnswerError::InvalidChooseOneOption(
            item_id,
            value.value(),
        ));
    }

    Ok(())
}

fn verify_item_choose_many(
    item_id: FormItemId,
    form_choose_many: FormItemChooseMany,
    answer_choose_many: FormAnswerItemChooseMany,
) -> Result<(), VerifyFormAnswerError> {
    let values: Vec<FormItemOption> = answer_choose_many
        .value()
        .into_iter()
        .map(FormItemOption::new)
        .collect();

    for value in &values {
        if !form_choose_many.options().contains(value) {
            return Err(VerifyFormAnswerError::InvalidChooseManyOption(
                item_id,
                value.clone().value(),
            ));
        }
    }

    if let Some(min) = form_choose_many.min_selection() {
        let min = min.clone().value();
        if values.len() < min as usize {
            return Err(VerifyFormAnswerError::TooFewOptionsChooseMany(item_id, min));
        }
    }

    if let Some(max) = form_choose_many.max_selection() {
        let max = max.clone().value();
        if values.len() > max as usize {
            return Err(VerifyFormAnswerError::TooManyOptionsChooseMany(
                item_id, max,
            ));
        }
    }

    Ok(())
}

fn verify_item_file(
    item_id: FormItemId,
    form_file: FormItemFile,
    answer_file: FormAnswerItemFile,
) -> Result<(), VerifyFormAnswerError> {
    let files = answer_file.clone().value();

    // extensionsはフロントエンドでチェックするためここでは何もしない

    if let Some(limit) = form_file.limit() {
        let limit = limit.clone().value();
        if files.len() > limit as usize {
            return Err(VerifyFormAnswerError::TooManyFiles(item_id, limit));
        }
    }
    Ok(())
}
