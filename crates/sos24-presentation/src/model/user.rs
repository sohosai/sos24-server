use serde::{Deserialize, Serialize};
use sos24_use_case::dto::user::{CreateUserDto, UserCategoryDto};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub kana_name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub category: UserCategory,
}

impl From<CreateUser> for CreateUserDto {
    fn from(value: CreateUser) -> Self {
        CreateUserDto::new(
            value.name,
            value.kana_name,
            value.email,
            value.password,
            value.phone_number,
            UserCategoryDto::from(value.category),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserCategory {
    UndergraduateStudent,
    GraduateStudent,
    AcademicStaff,
}

impl From<UserCategory> for UserCategoryDto {
    fn from(value: UserCategory) -> Self {
        match value {
            UserCategory::UndergraduateStudent => UserCategoryDto::UndergraduateStudent,
            UserCategory::GraduateStudent => UserCategoryDto::GraduateStudent,
            UserCategory::AcademicStaff => UserCategoryDto::AcademicStaff,
        }
    }
}
