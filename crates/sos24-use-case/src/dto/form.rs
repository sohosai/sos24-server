use sos24_domain::entity::form::FormItemExtension;
use sos24_domain::entity::{
    common::{date::WithDate, datetime::DateTime},
    form::{
        Form, FormDescription, FormItem, FormItemAllowNewline, FormItemDescription, FormItemKind,
        FormItemLimit, FormItemMax, FormItemMaxLength, FormItemMaxSelection, FormItemMin,
        FormItemMinLength, FormItemMinSelection, FormItemName, FormItemOption, FormItemRequired,
        FormTitle,
    },
};

use crate::dto::project::{ProjectAttributeDto, ProjectCategoryDto};
use crate::interactor::form::FormUseCaseError;

use super::{FromEntity, ToEntity};

#[derive(Debug)]
pub struct CreateFormDto {
    title: String,
    description: String,
    starts_at: String,
    ends_at: String,
    categories: Vec<ProjectCategoryDto>,
    attributes: Vec<ProjectAttributeDto>,
    items: Vec<NewFormItemDto>,
}

impl CreateFormDto {
    pub fn new(
        title: String,
        description: String,
        starts_at: String,
        ends_at: String,
        categories: Vec<ProjectCategoryDto>,
        attributes: Vec<ProjectAttributeDto>,
        items: Vec<NewFormItemDto>,
    ) -> Self {
        Self {
            title,
            description,
            starts_at,
            ends_at,
            categories,
            attributes,
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
            self.categories.into_entity()?,
            self.attributes.into_entity()?,
            self.items
                .into_iter()
                .map(NewFormItemDto::into_entity)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[derive(Debug)]
pub struct NewFormItemDto {
    name: String,
    description: String,
    required: bool,
    kind: FormItemKindDto,
}

impl NewFormItemDto {
    pub fn new(name: String, description: String, required: bool, kind: FormItemKindDto) -> Self {
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
            FormItemDescription::new(self.description),
            FormItemRequired::new(self.required),
            self.kind.into_entity()?,
        ))
    }
}

#[derive(Debug)]
pub struct UpdateFormDto {
    pub id: String,
    pub title: String,
    pub description: String,
    pub starts_at: String,
    pub ends_at: String,
    pub categories: Vec<ProjectCategoryDto>,
    pub attributes: Vec<ProjectAttributeDto>,
    pub items: Vec<NewFormItemDto>,
}

impl UpdateFormDto {
    pub fn new(
        id: String,
        title: String,
        description: String,
        starts_at: String,
        ends_at: String,
        categories: Vec<ProjectCategoryDto>,
        attributes: Vec<ProjectAttributeDto>,
        items: Vec<NewFormItemDto>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            starts_at,
            ends_at,
            categories,
            attributes,
            items,
        }
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
            categories: Vec::from_entity(form.categories),
            attributes: Vec::from_entity(form.attributes),
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
    pub id: String,
    pub name: String,
    pub description: String,
    pub required: bool,
    pub kind: FormItemKindDto,
}

impl FormItemDto {
    pub fn new(
        id: String,
        name: String,
        description: String,
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
            entity.id.value().to_string(),
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
