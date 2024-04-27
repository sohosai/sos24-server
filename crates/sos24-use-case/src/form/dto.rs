use sos24_domain::entity::form::FormItemExtension;
use sos24_domain::entity::form_answer::FormAnswer;
use sos24_domain::entity::{
    common::date::WithDate,
    form::{
        Form, FormItem, FormItemAllowNewline, FormItemDescription, FormItemKind, FormItemLimit,
        FormItemMax, FormItemMaxLength, FormItemMaxSelection, FormItemMin, FormItemMinLength,
        FormItemMinSelection, FormItemName, FormItemOption, FormItemRequired,
    },
};

use crate::form::FormUseCaseError;
use crate::project::dto::{ProjectAttributeDto, ProjectCategoryDto};
use crate::{FromEntity, ToEntity};

#[derive(Debug)]
pub struct NewFormItemDto {
    name: String,
    description: Option<String>,
    required: bool,
    kind: FormItemKindDto,
}

impl NewFormItemDto {
    pub fn new(
        name: String,
        description: Option<String>,
        required: bool,
        kind: FormItemKindDto,
    ) -> Self {
        Self {
            name,
            description,
            required,
            kind,
        }
    }
}

impl ToEntity for NewFormItemDto {
    type Entity = FormItem;
    type Error = FormUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        Ok(FormItem::create(
            FormItemName::new(self.name),
            self.description.map(FormItemDescription::new),
            FormItemRequired::new(self.required),
            self.kind.into_entity()?,
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
    pub categories: Vec<ProjectCategoryDto>,
    pub attributes: Vec<ProjectAttributeDto>,
    pub items: Vec<FormItemDto>,
    pub attachments: Vec<String>,
    pub answer_id: Option<String>,
    pub answered_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FromEntity for FormDto {
    type Entity = (WithDate<Form>, Option<WithDate<FormAnswer>>);
    fn from_entity((form_entity, form_answer_entity): Self::Entity) -> Self {
        let form = form_entity.value.destruct();
        let (answer_id, answered_at) = form_answer_entity
            .map(|form_answer_entity| {
                let answer = form_answer_entity.value.destruct();
                (Some(answer.id), Some(form_answer_entity.updated_at))
            })
            .unwrap_or((None, None));

        Self {
            id: form.id.value().to_string(),
            title: form.title.value(),
            description: form.description.value(),
            starts_at: form.starts_at.value(),
            ends_at: form.ends_at.value(),
            categories: Vec::from_entity(form.categories),
            attributes: Vec::from_entity(form.attributes),
            items: form
                .items
                .into_iter()
                .map(FormItemDto::from_entity)
                .collect(),
            attachments: form
                .attachments
                .into_iter()
                .map(|it| it.value().to_string())
                .collect(),
            answer_id: answer_id.map(|it| it.value().to_string()),
            answered_at,
            created_at: form_entity.created_at,
            updated_at: form_entity.updated_at,
            deleted_at: form_entity.deleted_at,
        }
    }
}

#[derive(Debug)]
pub struct FormSummaryDto {
    pub id: String,
    pub title: String,
    pub description: String,
    pub starts_at: chrono::DateTime<chrono::Utc>,
    pub ends_at: chrono::DateTime<chrono::Utc>,
    pub categories: Vec<ProjectCategoryDto>,
    pub attributes: Vec<ProjectAttributeDto>,
    pub answer_id: Option<String>,
    pub answered_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl FromEntity for FormSummaryDto {
    type Entity = (WithDate<Form>, Option<WithDate<FormAnswer>>);
    fn from_entity((form_entity, form_answer_entity): Self::Entity) -> Self {
        let form = form_entity.value.destruct();
        let (answer_id, answered_at) = form_answer_entity
            .map(|form_answer_entity| {
                let answer = form_answer_entity.value.destruct();
                (Some(answer.id), Some(form_answer_entity.updated_at))
            })
            .unwrap_or((None, None));

        Self {
            id: form.id.value().to_string(),
            title: form.title.value(),
            description: form.description.value(),
            starts_at: form.starts_at.value(),
            ends_at: form.ends_at.value(),
            categories: Vec::from_entity(form.categories),
            attributes: Vec::from_entity(form.attributes),
            answer_id: answer_id.map(|it| it.value().to_string()),
            answered_at,
            updated_at: form_entity.updated_at,
        }
    }
}

#[derive(Debug)]
pub struct FormItemDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    pub kind: FormItemKindDto,
}

impl FormItemDto {
    pub fn new(
        id: String,
        name: String,
        description: Option<String>,
        required: bool,
        kind: FormItemKindDto,
    ) -> Self {
        Self {
            id,
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
        Ok(FormItem::create(
            FormItemName::new(self.name),
            self.description.map(FormItemDescription::new),
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
            entity.id.value().to_string(),
            entity.name.value(),
            entity.description.map(|it| it.value()),
            entity.required.value(),
            FormItemKindDto::from_entity(entity.kind),
        )
    }
}

#[derive(Debug)]
pub enum FormItemKindDto {
    String {
        min_length: Option<i32>,
        max_length: Option<i32>,
        allow_newline: bool,
    },
    Int {
        min: Option<i32>,
        max: Option<i32>,
    },
    ChooseOne {
        options: Vec<String>,
    },
    ChooseMany {
        options: Vec<String>,
        min_selection: Option<i32>,
        max_selection: Option<i32>,
    },
    File {
        extensions: Option<Vec<String>>,
        limit: Option<i32>,
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
            } => Ok(FormItemKind::new_string(
                min_length.map(FormItemMinLength::new),
                max_length.map(FormItemMaxLength::new),
                FormItemAllowNewline::new(allow_newline),
            )),
            FormItemKindDto::Int { min, max } => Ok(FormItemKind::new_int(
                min.map(FormItemMin::new),
                max.map(FormItemMax::new),
            )),
            FormItemKindDto::ChooseOne { options } => Ok(FormItemKind::new_choose_one(
                options.into_iter().map(FormItemOption::new).collect(),
            )),
            FormItemKindDto::ChooseMany {
                options,
                min_selection,
                max_selection,
            } => Ok(FormItemKind::new_choose_many(
                options.into_iter().map(FormItemOption::new).collect(),
                min_selection.map(FormItemMinSelection::new),
                max_selection.map(FormItemMaxSelection::new),
            )),
            FormItemKindDto::File { extensions, limit } => Ok(FormItemKind::new_file(
                extensions.map(|it| it.into_iter().map(FormItemExtension::new).collect()),
                limit.map(FormItemLimit::new),
            )),
        }
    }
}

impl FromEntity for FormItemKindDto {
    type Entity = FormItemKind;
    fn from_entity(entity: Self::Entity) -> Self {
        match entity {
            FormItemKind::String(item) => {
                let item = item.destruct();
                Self::String {
                    min_length: item.min_length.map(|it| it.value()),
                    max_length: item.max_length.map(|it| it.value()),
                    allow_newline: item.allow_newline.value(),
                }
            }
            FormItemKind::Int(item) => {
                let item = item.destruct();
                Self::Int {
                    min: item.min.map(|it| it.value()),
                    max: item.max.map(|it| it.value()),
                }
            }
            FormItemKind::ChooseOne(item) => {
                let item = item.destruct();
                Self::ChooseOne {
                    options: item.options.into_iter().map(|it| it.value()).collect(),
                }
            }
            FormItemKind::ChooseMany(item) => {
                let item = item.destruct();
                Self::ChooseMany {
                    options: item.options.into_iter().map(|it| it.value()).collect(),
                    min_selection: item.min_selection.map(|it| it.value()),
                    max_selection: item.max_selection.map(|it| it.value()),
                }
            }
            FormItemKind::File(item) => {
                let item = item.destruct();
                Self::File {
                    extensions: item
                        .extensions
                        .map(|it| it.into_iter().map(|it| it.value()).collect()),
                    limit: item.limit.map(|it| it.value()),
                }
            }
        }
    }
}
