use std::str::FromStr;

use bitflags::bitflags;
use getset::Getters;
use thiserror::Error;

use crate::{ensure, impl_value_object};

use super::{
    actor::Actor,
    permission::{PermissionDeniedError, Permissions},
    user::UserId,
};

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
    remarks: Option<ProjectRemarks>,
}

impl Project {
    #[allow(clippy::too_many_arguments)]
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
        remarks: Option<ProjectRemarks>,
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
            remarks: None,
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
    pub remarks: Option<ProjectRemarks>,
}

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Already owner or sub-owner")]
    AlreadyOwnerOrSubOwner,
}

impl Project {
    pub fn is_owned_by(&self, user_id: &UserId) -> bool {
        self.owner_id() == user_id
            || self
                .sub_owner_id()
                .as_ref()
                .map_or(false, |sub_owner_id| sub_owner_id == user_id)
    }

    pub fn is_visible_to(&self, actor: &Actor) -> bool {
        self.is_owned_by(actor.user_id()) || actor.has_permission(Permissions::READ_PROJECT_ALL)
    }

    pub fn is_updatable_by(&self, actor: &Actor) -> bool {
        self.is_owned_by(actor.user_id()) || actor.has_permission(Permissions::UPDATE_PROJECT_ALL)
    }

    pub fn set_title(
        &mut self,
        actor: &Actor,
        title: ProjectTitle,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.title = title;
        Ok(())
    }

    pub fn set_kana_title(
        &mut self,
        actor: &Actor,
        kana_title: ProjectKanaTitle,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.kana_title = kana_title;
        Ok(())
    }

    pub fn set_group_name(
        &mut self,
        actor: &Actor,
        group_name: ProjectGroupName,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.group_name = group_name;
        Ok(())
    }

    pub fn set_kana_group_name(
        &mut self,
        actor: &Actor,
        kana_group_name: ProjectKanaGroupName,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.kana_group_name = kana_group_name;
        Ok(())
    }

    pub fn set_category(
        &mut self,
        actor: &Actor,
        category: ProjectCategory,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.category = category;
        Ok(())
    }

    pub fn set_attributes(
        &mut self,
        actor: &Actor,
        attributes: ProjectAttributes,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.attributes = attributes;
        Ok(())
    }

    pub fn set_remarks(
        &mut self,
        actor: &Actor,
        remarks: ProjectRemarks,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(actor.has_permission(Permissions::UPDATE_PROJECT_ALL));
        self.remarks.replace(remarks);
        Ok(())
    }

    pub fn set_owner_id(&mut self, id: UserId) -> Result<(), ProjectError> {
        if id == self.owner_id
            || self
                .sub_owner_id
                .as_ref()
                .map(|sub_owner_id| sub_owner_id == &id)
                .unwrap_or(false)
        {
            return Err(ProjectError::AlreadyOwnerOrSubOwner);
        }

        self.owner_id = id;
        Ok(())
    }

    pub fn set_sub_owner_id(&mut self, id: UserId) -> Result<(), ProjectError> {
        if id == self.owner_id
            || self
                .sub_owner_id
                .as_ref()
                .map(|sub_owner_id| sub_owner_id == &id)
                .unwrap_or(false)
        {
            return Err(ProjectError::AlreadyOwnerOrSubOwner);
        }

        self.sub_owner_id.replace(id);
        Ok(())
    }
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

// 最大`MAXLEN`文字の文字列を持つ
// 半角・全角英数字及び半角記号は3文字で仮名2文字としてカウントする
// 絵文字は含めることができない
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedString<const MAXLEN: usize>(String);

impl<const MAXLEN: usize> BoundedString<MAXLEN> {
    pub fn value(self) -> String {
        self.0
    }
}

#[derive(Debug, Error)]
pub enum BoundedStringError {
    #[error("Invalid character: `{0}`")]
    InvalidCharacter(char),
    #[error("Empty string is not allowed")]
    Empty,
    #[error("Too long (max: {0})")]
    TooLong(usize),
}

impl<const MAXLEN: usize> TryFrom<String> for BoundedString<MAXLEN> {
    type Error = BoundedStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut length = 0; // 文字列長を3倍してカウントする

        for c in value.chars() {
            if emojis::get(&c.to_string()).is_some() {
                return Err(BoundedStringError::InvalidCharacter(c));
            }

            let char_length = match c {
                '\u{0021}'..='\u{007E}' // 半角英数字・記号
                | '\u{FF10}'..='\u{FF19}' // 全角数字
                | '\u{FF21}'..='\u{FF3A}' // 全角英語（大文字）
                | '\u{FF41}'..='\u{FF5A}' // 全角英語（小文字）
                => 2,
                _ => 3,
            };

            length += char_length;
        }

        if length == 0 {
            return Err(BoundedStringError::Empty);
        }
        if length > MAXLEN * 3 {
            return Err(BoundedStringError::TooLong(MAXLEN));
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectTitle(BoundedString<20>);

impl ProjectTitle {
    pub fn value(self) -> String {
        self.0.value()
    }
}

impl TryFrom<String> for ProjectTitle {
    type Error = BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        BoundedString::try_from(value).map(Self)
    }
}

impl_value_object!(ProjectKanaTitle(String));

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectGroupName(BoundedString<20>);

impl ProjectGroupName {
    pub fn value(self) -> String {
        self.0.value()
    }
}

impl TryFrom<String> for ProjectGroupName {
    type Error = BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        BoundedString::try_from(value).map(Self)
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectAttributes(u32);

bitflags! {
    impl ProjectAttributes: u32 {
        const ACADEMIC = 1 << 0;
        const ART = 1 << 1;
        const OFFICIAL = 1 << 2;
        const INSIDE = 1 << 3;
        const OUTSIDE = 1 << 4;
    }
}

impl_value_object!(ProjectRemarks(String));

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectCategories(u32);

bitflags! {
    impl ProjectCategories: u32 {
        const GENERAL = 1 << 0;
        const FOODS_WITH_KITCHEN = 1 << 1;
        const FOODS_WITHOUT_KITCHEN = 1 << 2;
        const FOODS_WITHOUT_COOKING = 1 << 3;
        const STAGE_1A = 1 << 4;
        const STAGE_UNIVERSITY_HALL = 1 << 5;
        const STAGE_UNITED = 1 << 6;
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::project::ProjectTitle;

    #[test]
    fn valid_project_title() {
        let kana20 = "あ".repeat(20);
        assert!(ProjectTitle::try_from(kana20).is_ok());

        let kana18 = "あ".repeat(18);
        assert!(ProjectTitle::try_from(format!("{kana18}AAA")).is_ok());
    }

    #[test]
    fn invalid_project_title() {
        assert!(ProjectTitle::try_from("".to_string()).is_err());

        let kana21 = "あ".repeat(21);
        assert!(ProjectTitle::try_from(kana21).is_err());

        let kana18 = "あ".repeat(18);
        assert!(ProjectTitle::try_from(format!("{kana18}AAAA")).is_err());

        assert!(ProjectTitle::try_from("🙂".to_string()).is_err());
    }
}
