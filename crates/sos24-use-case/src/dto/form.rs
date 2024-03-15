use sos24_domain::entity::{
    common::datetime::DateTime,
    form::{
        Form, FormDescription, FormItem, FormItemAllowNewline, FormItemDescription,
        FormItemExtention, FormItemKind, FormItemLimit, FormItemMax, FormItemMaxLength,
        FormItemMaxSelection, FormItemMin, FormItemMinLength, FormItemMinSelection, FormItemName,
        FormItemOption, FormItemRequired, FormTitle,
    },
};

use crate::interactor::form::FormUseCaseError;

use super::ToEntity;

#[derive(Debug)]
pub struct CreateFormDto {
    title: String,
    description: String,
    starts_at: String,
    ends_at: String,
    items: Vec<CreateFormItemDto>,
}

impl CreateFormDto {
    pub fn new(
        title: String,
        description: String,
        starts_at: String,
        ends_at: String,
        items: Vec<CreateFormItemDto>,
    ) -> Self {
        Self {
            title,
            description,
            starts_at,
            ends_at,
            items,
        }
    }
}

impl ToEntity for CreateFormDto {
    type Entity = Form;
    type Error = FormUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        Ok(Form::create(
            FormTitle::new(self.title),
            FormDescription::new(self.description),
            DateTime::try_from(self.starts_at)?,
            DateTime::try_from(self.ends_at)?,
            self.items
                .into_iter()
                .map(CreateFormItemDto::into_entity)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[derive(Debug)]
pub struct CreateFormItemDto {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub kind: FormItemKindDto,
}

impl CreateFormItemDto {
    pub fn new(name: String, description: String, required: bool, kind: FormItemKindDto) -> Self {
        Self {
            name,
            description,
            required,
            kind,
        }
    }
}

impl ToEntity for CreateFormItemDto {
    type Entity = FormItem;
    type Error = FormUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        Ok(FormItem::new(
            FormItemName::new(self.name),
            FormItemDescription::new(self.description),
            FormItemRequired::new(self.required),
            self.kind.into_entity()?,
        ))
    }
}

#[derive(Debug)]
pub enum FormItemKindDto {
    String {
        min_length: i32,
        max_length: i32,
        allow_newline: bool,
    },
    Int {
        min: i32,
        max: i32,
    },
    ChooseOne {
        options: Vec<String>,
    },
    ChooseMany {
        options: Vec<String>,
        min_selection: i32,
        max_selection: i32,
    },
    File {
        extentions: Vec<String>,
        limit: i32,
    },
}

impl ToEntity for FormItemKindDto {
    type Entity = FormItemKind;
    type Error = FormUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        match self {
            FormItemKindDto::String {
                min_length,
                max_length,
                allow_newline,
            } => Ok(FormItemKind::String {
                min_length: FormItemMinLength::new(min_length),
                max_length: FormItemMaxLength::new(max_length),
                allow_newline: FormItemAllowNewline::new(allow_newline),
            }),
            FormItemKindDto::Int { min, max } => Ok(FormItemKind::Int {
                min: FormItemMin::new(min),
                max: FormItemMax::new(max),
            }),
            FormItemKindDto::ChooseOne { options } => Ok(FormItemKind::ChooseOne {
                options: options.into_iter().map(FormItemOption::new).collect(),
            }),
            FormItemKindDto::ChooseMany {
                options,
                min_selection,
                max_selection,
            } => Ok(FormItemKind::ChooseMany {
                options: options.into_iter().map(FormItemOption::new).collect(),
                min_selection: FormItemMinSelection::new(min_selection),
                max_selection: FormItemMaxSelection::new(max_selection),
            }),
            FormItemKindDto::File { extentions, limit } => Ok(FormItemKind::File {
                extentions: extentions.into_iter().map(FormItemExtention::new).collect(),
                limit: FormItemLimit::new(limit),
            }),
        }
    }
}
