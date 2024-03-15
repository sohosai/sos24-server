use sos24_domain::entity::{
    common::{date::WithDate, datetime::DateTime},
    form::{
        Form, FormDescription, FormItem, FormItemAllowNewline, FormItemDescription,
        FormItemExtention, FormItemKind, FormItemLimit, FormItemMax, FormItemMaxLength,
        FormItemMaxSelection, FormItemMin, FormItemMinLength, FormItemMinSelection, FormItemName,
        FormItemOption, FormItemRequired, FormTitle,
    },
};

use crate::interactor::form::FormUseCaseError;

use super::{FromEntity, ToEntity};

#[derive(Debug)]
pub struct CreateFormDto {
    title: String,
    description: String,
    starts_at: String,
    ends_at: String,
    items: Vec<FormItemDto>,
}

impl CreateFormDto {
    pub fn new(
        title: String,
        description: String,
        starts_at: String,
        ends_at: String,
        items: Vec<FormItemDto>,
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
                .map(FormItemDto::into_entity)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[derive(Debug)]
pub struct FormDto {
    pub id: String,
    pub title: String,
    pub description: String,
    pub starts_at: chrono::DateTime<chrono::Utc>,
    pub ends_at: chrono::DateTime<chrono::Utc>,
    pub items: Vec<FormItemDto>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FromEntity for FormDto {
    type Entity = WithDate<Form>;
    fn from_entity(entity: Self::Entity) -> Self {
        let form = entity.value.destruct();
        Self {
            id: form.id.value().to_string(),
            title: form.title.value(),
            description: form.description.value(),
            starts_at: form.starts_at.value(),
            ends_at: form.ends_at.value(),
            items: form
                .items
                .into_iter()
                .map(FormItemDto::from_entity)
                .collect(),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}

#[derive(Debug)]
pub struct FormItemDto {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub kind: FormItemKindDto,
}

impl FormItemDto {
    pub fn new(name: String, description: String, required: bool, kind: FormItemKindDto) -> Self {
        Self {
            name,
            description,
            required,
            kind,
        }
    }
}

impl ToEntity for FormItemDto {
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

impl FromEntity for FormItemDto {
    type Entity = FormItem;
    fn from_entity(entity: Self::Entity) -> Self {
        let entity = entity.destruct();
        Self::new(
            entity.name.value(),
            entity.description.value(),
            entity.required.value(),
            FormItemKindDto::from_entity(entity.kind),
        )
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

impl FromEntity for FormItemKindDto {
    type Entity = FormItemKind;
    fn from_entity(entity: Self::Entity) -> Self {
        match entity {
            FormItemKind::String {
                min_length,
                max_length,
                allow_newline,
            } => FormItemKindDto::String {
                min_length: min_length.value(),
                max_length: max_length.value(),
                allow_newline: allow_newline.value(),
            },
            FormItemKind::Int { min, max } => FormItemKindDto::Int {
                min: min.value(),
                max: max.value(),
            },
            FormItemKind::ChooseOne { options } => FormItemKindDto::ChooseOne {
                options: options.into_iter().map(|o| o.value()).collect(),
            },
            FormItemKind::ChooseMany {
                options,
                min_selection,
                max_selection,
            } => FormItemKindDto::ChooseMany {
                options: options.into_iter().map(|o| o.value()).collect(),
                min_selection: min_selection.value(),
                max_selection: max_selection.value(),
            },
            FormItemKind::File { extentions, limit } => FormItemKindDto::File {
                extentions: extentions.into_iter().map(|e| e.value()).collect(),
                limit: limit.value(),
            },
        }
    }
}
