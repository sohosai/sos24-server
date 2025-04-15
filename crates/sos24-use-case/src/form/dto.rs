use sos24_domain::entity::form::{
    Form, FormItem, FormItemAllowNewline, FormItemDescription, FormItemKind, FormItemLimit,
    FormItemMax, FormItemMaxLength, FormItemMaxSelection, FormItemMin, FormItemMinLength,
    FormItemMinSelection, FormItemName, FormItemOption, FormItemRequired,
};
use sos24_domain::entity::form::{FormIsDraft, FormItemExtension};
use sos24_domain::entity::form_answer::FormAnswer;

use crate::project::dto::{ProjectAttributesDto, ProjectCategoriesDto};

use super::FormUseCaseError;

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

impl TryFrom<NewFormItemDto> for FormItem {
    type Error = FormUseCaseError;
    fn try_from(value: NewFormItemDto) -> Result<Self, Self::Error> {
        Ok(FormItem::create(
            FormItemName::new(value.name),
            value.description.map(FormItemDescription::new),
            FormItemRequired::new(value.required),
            FormItemKind::try_from(value.kind)?,
        ))
    }
}

#[derive(Debug)]
pub struct FormIsDraftDto(bool);

impl FormIsDraftDto {
    pub fn new(value: bool) -> Self {
        FormIsDraftDto(value)
    }

    pub fn value(&self) -> bool {
        self.0
    }
}

impl From<FormIsDraft> for FormIsDraftDto {
    fn from(value: FormIsDraft) -> Self {
        FormIsDraftDto(value.value())
    }
}

impl From<FormIsDraftDto> for FormIsDraft {
    fn from(value: FormIsDraftDto) -> Self {
        FormIsDraft::new(value.value())
    }
}

#[derive(Debug)]
pub struct FormDto {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_draft: FormIsDraftDto,
    pub starts_at: chrono::DateTime<chrono::Utc>,
    pub ends_at: chrono::DateTime<chrono::Utc>,
    pub categories: ProjectCategoriesDto,
    pub attributes: ProjectAttributesDto,
    pub items: Vec<FormItemDto>,
    pub attachments: Vec<String>,
    pub answer_id: Option<String>,
    pub answered_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<(Form, Option<FormAnswer>)> for FormDto {
    fn from((form, form_answer): (Form, Option<FormAnswer>)) -> Self {
        let form = form.destruct();
        let (answer_id, answered_at) = form_answer
            .map(|form_answer| {
                let form_answer = form_answer.destruct();
                (Some(form_answer.id), Some(form_answer.created_at))
            })
            .unwrap_or((None, None));

        Self {
            id: form.id.value().to_string(),
            title: form.title.value(),
            description: form.description.value(),
            is_draft: FormIsDraftDto::from(form.is_draft),
            starts_at: form.starts_at.value(),
            ends_at: form.ends_at.value(),
            categories: ProjectCategoriesDto::from(form.categories),
            attributes: ProjectAttributesDto::from(form.attributes),
            items: form.items.into_iter().map(FormItemDto::from).collect(),
            attachments: form
                .attachments
                .into_iter()
                .map(|it| it.value().to_string())
                .collect(),
            answer_id: answer_id.map(|it| it.value().to_string()),
            answered_at: answered_at.map(|it| it.value()),
            created_at: form.created_at.value(),
            updated_at: form.updated_at.value(),
        }
    }
}

#[derive(Debug)]
pub struct FormSummaryDto {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_draft: FormIsDraftDto,
    pub starts_at: chrono::DateTime<chrono::Utc>,
    pub ends_at: chrono::DateTime<chrono::Utc>,
    pub categories: ProjectCategoriesDto,
    pub attributes: ProjectAttributesDto,
    pub answer_id: Option<String>,
    pub answered_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<(Form, Option<FormAnswer>)> for FormSummaryDto {
    fn from((form, form_answer): (Form, Option<FormAnswer>)) -> Self {
        let form = form.destruct();
        let (answer_id, answered_at) = form_answer
            .map(|form_answer| {
                let form_answer = form_answer.destruct();
                (Some(form_answer.id), Some(form_answer.created_at))
            })
            .unwrap_or((None, None));

        Self {
            id: form.id.value().to_string(),
            title: form.title.value(),
            description: form.description.value(),
            is_draft: FormIsDraftDto::from(form.is_draft),
            starts_at: form.starts_at.value(),
            ends_at: form.ends_at.value(),
            categories: ProjectCategoriesDto::from(form.categories),
            attributes: ProjectAttributesDto::from(form.attributes),
            answer_id: answer_id.map(|it| it.value().to_string()),
            answered_at: answered_at.map(|it| it.value()),
            updated_at: form.updated_at.value(),
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

impl TryFrom<FormItemDto> for FormItem {
    type Error = FormUseCaseError;
    fn try_from(value: FormItemDto) -> Result<Self, Self::Error> {
        Ok(FormItem::create(
            FormItemName::new(value.name),
            value.description.map(FormItemDescription::new),
            FormItemRequired::new(value.required),
            FormItemKind::try_from(value.kind)?,
        ))
    }
}

impl From<FormItem> for FormItemDto {
    fn from(entity: FormItem) -> Self {
        let entity = entity.destruct();
        Self::new(
            entity.id.value().to_string(),
            entity.name.value(),
            entity.description.map(|it| it.value()),
            entity.required.value(),
            FormItemKindDto::from(entity.kind),
        )
    }
}

#[derive(Debug)]
pub enum FormItemKindDto {
    String {
        min_length: Option<u32>,
        max_length: Option<u32>,
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
        min_selection: Option<u32>,
        max_selection: Option<u32>,
    },
    File {
        extensions: Option<Vec<String>>,
        limit: Option<u32>,
    },
}

impl TryFrom<FormItemKindDto> for FormItemKind {
    type Error = FormUseCaseError;
    fn try_from(value: FormItemKindDto) -> Result<Self, Self::Error> {
        match value {
            FormItemKindDto::String {
                min_length,
                max_length,
                allow_newline,
            } => Ok(FormItemKind::new_string(
                min_length.map(FormItemMinLength::new),
                max_length.map(FormItemMaxLength::new),
                FormItemAllowNewline::new(allow_newline),
            )?),
            FormItemKindDto::Int { min, max } => Ok(FormItemKind::new_int(
                min.map(FormItemMin::new),
                max.map(FormItemMax::new),
            )?),
            FormItemKindDto::ChooseOne { options } => Ok(FormItemKind::new_choose_one(
                options.into_iter().map(FormItemOption::new).collect(),
            )?),
            FormItemKindDto::ChooseMany {
                options,
                min_selection,
                max_selection,
            } => Ok(FormItemKind::new_choose_many(
                options.into_iter().map(FormItemOption::new).collect(),
                min_selection.map(FormItemMinSelection::new),
                max_selection.map(FormItemMaxSelection::new),
            )?),
            FormItemKindDto::File { extensions, limit } => Ok(FormItemKind::new_file(
                extensions.map(|it| it.into_iter().map(FormItemExtension::new).collect()),
                limit.map(FormItemLimit::new),
            )),
        }
    }
}

impl From<FormItemKind> for FormItemKindDto {
    fn from(entity: FormItemKind) -> Self {
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
