use std::str::FromStr;

use getset::Getters;
use thiserror::Error;

use crate::impl_value_object;

use super::user::UserId;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct Project {
    #[getset(get = "pub")]
    id: ProjectId,
    #[getset(get = "pub")]
    index: ProjectIndex,
    #[getset(get = "pub")]
    title: ProjectTitle,
    #[getset(get = "pub")]
    kana_title: ProjectKanaTitle,
    #[getset(get = "pub")]
    group_name: ProjectGroupName,
    #[getset(get = "pub")]
    kana_group_name: ProjectKanaGroupName,
    #[getset(get = "pub")]
    category: ProjectCategory,
    #[getset(get = "pub")]
    attributes: ProjectAttributes,
    #[getset(get = "pub")]
    owner_id: UserId,
    #[getset(get = "pub")]
    sub_owner_id: Option<UserId>,
    #[getset(get = "pub")]
    remarks: ProjectRemarks,
}

impl Project {
    pub fn new(
        id: ProjectId,
        index: ProjectIndex,
        title: ProjectTitle,
        kana_title: ProjectKanaTitle,
        group_name: ProjectGroupName,
        kana_group_name: ProjectKanaGroupName,
        category: ProjectCategory,
        attributes: ProjectAttributes,
        owner_id: UserId,
        sub_owner_id: Option<UserId>,
        remarks: ProjectRemarks,
    ) -> Self {
        Self {
            id,
            index,
            title,
            kana_title,
            group_name,
            kana_group_name,
            category,
            attributes,
            owner_id,
            sub_owner_id,
            remarks,
        }
    }

    pub fn create(
        title: ProjectTitle,
        kana_title: ProjectKanaTitle,
        group_name: ProjectGroupName,
        kana_group_name: ProjectKanaGroupName,
        category: ProjectCategory,
        attributes: ProjectAttributes,
        owner_id: UserId,
    ) -> Self {
        Self {
            id: ProjectId::new(uuid::Uuid::new_v4()),
            index: ProjectIndex::new(0), // TODO
            title,
            kana_title,
            group_name,
            kana_group_name,
            category,
            attributes,
            owner_id,
            sub_owner_id: None,
            remarks: ProjectRemarks::new(String::new()),
        }
    }

    pub fn destruct(self) -> DestructedProject {
        DestructedProject {
            id: self.id,
            index: self.index,
            title: self.title,
            kana_title: self.kana_title,
            group_name: self.group_name,
            kana_group_name: self.kana_group_name,
            category: self.category,
            attributes: self.attributes,
            owner_id: self.owner_id,
            sub_owner_id: self.sub_owner_id,
            remarks: self.remarks,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructedProject {
    pub id: ProjectId,
    pub index: ProjectIndex,
    pub title: ProjectTitle,
    pub kana_title: ProjectKanaTitle,
    pub group_name: ProjectGroupName,
    pub kana_group_name: ProjectKanaGroupName,
    pub category: ProjectCategory,
    pub attributes: ProjectAttributes,
    pub owner_id: UserId,
    pub sub_owner_id: Option<UserId>,
    pub remarks: ProjectRemarks,
}

impl_value_object!(ProjectId(uuid::Uuid));
#[derive(Debug, Error)]
pub enum ProjectIdError {
    #[error("Invalid UUID")]
    InvalidUuid,
}
impl TryFrom<String> for ProjectId {
    type Error = ProjectIdError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let uuid = uuid::Uuid::from_str(&value).map_err(|_| ProjectIdError::InvalidUuid)?;
        Ok(Self(uuid))
    }
}

impl_value_object!(ProjectIndex(i32));
impl_value_object!(ProjectTitle(String));
impl_value_object!(ProjectKanaTitle(String));
impl_value_object!(ProjectGroupName(String));
impl_value_object!(ProjectKanaGroupName(String));

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectCategory {
    General,
    FoodsWithKitchen,
    FoodsWithoutKitchen,
    FoodsWithoutCooking,
    Stage1A,
    StageUniversityHall,
    StageUnited,
}

impl_value_object!(ProjectAttributes(i32));
impl_value_object!(ProjectRemarks(String));
