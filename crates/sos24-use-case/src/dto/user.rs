use std::convert::Infallible;

use sos24_domain::entity::user::{
    User, UserCategory, UserEmail, UserId, UserKanaName, UserName, UserPhoneNumber, UserRole,
};

use crate::error::{user::UserError, Result};

use super::ToEntity;

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
