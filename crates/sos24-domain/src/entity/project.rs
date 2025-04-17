use std::str::FromStr;

use bitflags::bitflags;
use getset::Getters;
use thiserror::Error;
use unicode_segmentation::UnicodeSegmentation;

use crate::{ensure, impl_value_object};

use super::{
    actor::Actor,
    common::datetime::DateTime,
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
    #[getset(get = "pub")]
    location_id: Option<ProjectLocationId>,
    #[getset(get = "pub")]
    created_at: DateTime,
    #[getset(get = "pub")]
    updated_at: DateTime,
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
        location_id: Option<ProjectLocationId>,
        created_at: DateTime,
        updated_at: DateTime,
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
            location_id,
            created_at,
            updated_at,
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
        let now = DateTime::now();
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
            location_id: None,
            created_at: now.clone(),
            updated_at: now,
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
            location_id: self.location_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
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
    pub location_id: Option<ProjectLocationId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
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

    pub fn set_location_id(
        &mut self,
        actor: &Actor,
        location_id: ProjectLocationId,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(actor.has_permission(Permissions::UPDATE_PROJECT_ALL));
        self.location_id.replace(location_id);
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
    InvalidCharacter(String),
    #[error("Empty string is not allowed")]
    Empty,
    #[error("Too long (max: {0})")]
    TooLong(usize),
}

impl<const MAXLEN: usize> TryFrom<String> for BoundedString<MAXLEN> {
    type Error = BoundedStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut length = 0; // 文字列長を3倍してカウントする

        let is_small = |c: char| match c {
            '\u{0021}'..='\u{007E}' // 半角英数字・記号
            | '\u{FF10}'..='\u{FF19}' // 全角数字
            | '\u{FF21}'..='\u{FF3A}' // 全角英語（大文字）
            | '\u{FF41}'..='\u{FF5A}' // 全角英語（小文字）
            => true,
            _ => false,
        };

        for grapheme_cluster in value.graphemes(true) {
            if emojis::get(grapheme_cluster).is_some() {
                return Err(BoundedStringError::InvalidCharacter(
                    grapheme_cluster.to_string(),
                ));
            }

            let mut chars = grapheme_cluster.chars();
            let is_small_char = chars
                .next()
                .map(|c| is_small(c) && chars.next().is_none())
                .unwrap_or(false);
            length += if is_small_char { 2 } else { 3 };
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectCategory {
    General,
    FoodsWithKitchen,
    FoodsWithoutKitchen,
    FoodsWithoutCooking,
    Stage1A,
    StageUniversityHall,
    StageUnited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl ProjectAttributes {
    pub fn matches(self, attributes: ProjectAttributes) -> bool {
        if self.contains(Self::OFFICIAL) {
            // フィルターに委員会開催企画が含まれている場合はORで判定する
            attributes.intersects(self)
        } else {
            // フィルターに委員会開催企画が含まれていない場合はORで判定し、かつ、委員会開催企画が含まれていない企画のみを対象とする
            attributes.intersects(self) && !attributes.contains(Self::OFFICIAL)
        }
    }
}

impl_value_object!(ProjectRemarks(String));

// 場所IDはIDといっても人間(主に総合計画局)が定めるもので、
// - 屋外では"[A-Z][0-9]"の2桁 ex) "A1"
// - 屋内では"[0-9][A-Z][0-9]{3}"の5桁(教室番号に一致) ex) "3C213"
// という規則があるが、変更の可能性や柔軟性を鑑みて、Stringで格納することとする
impl_value_object!(ProjectLocationId(String));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl ProjectCategories {
    pub fn matches(self, category: ProjectCategory) -> bool {
        match category {
            ProjectCategory::General => ProjectCategories::GENERAL.intersects(self),
            ProjectCategory::FoodsWithKitchen => {
                ProjectCategories::FOODS_WITH_KITCHEN.intersects(self)
            }
            ProjectCategory::FoodsWithoutKitchen => {
                ProjectCategories::FOODS_WITHOUT_KITCHEN.intersects(self)
            }
            ProjectCategory::FoodsWithoutCooking => {
                ProjectCategories::FOODS_WITHOUT_COOKING.intersects(self)
            }
            ProjectCategory::Stage1A => ProjectCategories::STAGE_1A.intersects(self),
            ProjectCategory::StageUniversityHall => {
                ProjectCategories::STAGE_UNIVERSITY_HALL.intersects(self)
            }
            ProjectCategory::StageUnited => ProjectCategories::STAGE_UNITED.intersects(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::project::{
        ProjectAttributes, ProjectCategories, ProjectCategory, ProjectTitle,
    };

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
        assert!(ProjectTitle::try_from("企画名#️⃣appare".to_string()).is_err());
    }

    #[test]
    fn match_project_category() {
        let categories = ProjectCategories::GENERAL | ProjectCategories::STAGE_1A;
        assert!(categories.matches(ProjectCategory::General));
        assert!(categories.matches(ProjectCategory::Stage1A));
        assert!(!categories.matches(ProjectCategory::FoodsWithKitchen));
    }

    #[test]
    fn not_match_project_category() {
        let categories = ProjectCategories::empty();
        assert!(!categories.matches(ProjectCategory::General));

        let categories = ProjectCategories::GENERAL | ProjectCategories::STAGE_1A;
        assert!(!categories.matches(ProjectCategory::FoodsWithKitchen));
    }

    #[test]
    fn match_project_attributes() {
        let filter_attributes = ProjectAttributes::ACADEMIC | ProjectAttributes::INSIDE;
        for attributes_bits in 0..ProjectAttributes::all().bits() {
            let attributes = ProjectAttributes::from_bits(attributes_bits).unwrap();
            if !attributes.contains(ProjectAttributes::OFFICIAL)
                && (attributes.contains(ProjectAttributes::ACADEMIC)
                    || attributes.contains(ProjectAttributes::INSIDE))
            {
                assert!(filter_attributes.matches(attributes));
            } else {
                assert!(!filter_attributes.matches(attributes));
            }
        }

        let filter_attributes = ProjectAttributes::OFFICIAL;
        assert!(filter_attributes.matches(ProjectAttributes::OFFICIAL));
        assert!(
            filter_attributes.matches(ProjectAttributes::OFFICIAL | ProjectAttributes::ACADEMIC)
        );
    }

    #[test]
    fn not_match_project_attributes() {
        let filter_attributes = ProjectAttributes::empty();
        assert!(!filter_attributes.matches(ProjectAttributes::ACADEMIC));

        let filter_attributes = ProjectAttributes::ACADEMIC | ProjectAttributes::INSIDE;
        assert!(!filter_attributes.matches(ProjectAttributes::OUTSIDE));

        let filter_attributes = ProjectAttributes::ACADEMIC;
        assert!(!filter_attributes.matches(ProjectAttributes::OFFICIAL));
        assert!(
            !filter_attributes.matches(ProjectAttributes::OFFICIAL | ProjectAttributes::ACADEMIC)
        );
    }
}
