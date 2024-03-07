use std::convert::Infallible;

use sos24_domain::entity::{
    common::date::WithDate,
    user::{
        User, UserCategory, UserEmail, UserId, UserKanaName, UserName, UserPhoneNumber, UserRole,
    },
};

use crate::error::{user::UserError, Result};

use super::{FromEntity, ToEntity};

#[derive(Debug)]
pub struct CreateUserDto {
    pub name: String,
    pub kana_name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub category: UserCategoryDto,
}

impl CreateUserDto {
    pub fn new(
        name: String,
        kana_name: String,
        email: String,
        password: String,
        phone_number: String,
        category: UserCategoryDto,
    ) -> Self {
        Self {
            name,
            kana_name,
            email,
            password,
            phone_number,
            category,
        }
    }
}

impl ToEntity for (String, CreateUserDto) {
    type Entity = User;
    type Error = UserError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        let (id, user) = self;
        Ok(User::new_general(
            UserId::new(id),
            UserName::new(user.name),
            UserKanaName::new(user.kana_name),
            UserEmail::try_from(user.email)?,
            UserPhoneNumber::new(user.phone_number),
            user.category.into_entity()?,
        ))
    }
}

#[derive(Debug)]
pub struct UserDto {
    pub id: String,
    pub name: String,
    pub kana_name: String,
    pub email: String,
    pub phone_number: String,
    pub role: UserRoleDto,
    pub category: UserCategoryDto,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FromEntity for UserDto {
    type Entity = WithDate<User>;
    fn from_entity(entity: Self::Entity) -> Self {
        Self {
            id: entity.value.id.value(),
            name: entity.value.name.value(),
            kana_name: entity.value.kana_name.value(),
            email: entity.value.email.value(),
            phone_number: entity.value.phone_number.value(),
            role: UserRoleDto::from_entity(entity.value.role),
            category: UserCategoryDto::from_entity(entity.value.category),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}

#[derive(Debug)]
pub enum UserRoleDto {
    General,
}

impl ToEntity for UserRoleDto {
    type Entity = UserRole;
    type Error = Infallible;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        Ok(match self {
            UserRoleDto::General => UserRole::General,
        })
    }
}

impl FromEntity for UserRoleDto {
    type Entity = UserRole;
    fn from_entity(entity: Self::Entity) -> Self {
        match entity {
            UserRole::General => UserRoleDto::General,
        }
    }
}

#[derive(Debug)]
pub enum UserCategoryDto {
    UndergraduateStudent,
    GraduateStudent,
    AcademicStaff,
}

impl ToEntity for UserCategoryDto {
    type Entity = UserCategory;
    type Error = Infallible;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        Ok(match self {
            UserCategoryDto::UndergraduateStudent => UserCategory::UndergraduateStudent,
            UserCategoryDto::GraduateStudent => UserCategory::GraduateStudent,
            UserCategoryDto::AcademicStaff => UserCategory::AcademicStaff,
        })
    }
}

impl FromEntity for UserCategoryDto {
    type Entity = UserCategory;
    fn from_entity(entity: Self::Entity) -> Self {
        match entity {
            UserCategory::UndergraduateStudent => UserCategoryDto::UndergraduateStudent,
            UserCategory::GraduateStudent => UserCategoryDto::GraduateStudent,
            UserCategory::AcademicStaff => UserCategoryDto::AcademicStaff,
        }
    }
}
