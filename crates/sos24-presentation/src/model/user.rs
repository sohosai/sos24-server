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

pub trait ConvertToCreateUserDto {
    fn to_create_user_dto(self) -> CreateUserDto;
}

impl ConvertToCreateUserDto for (String, CreateUser) {
    fn to_create_user_dto(self) -> CreateUserDto {
        let (id, user) = self;
        CreateUserDto::new(
            id,
            user.name,
            user.kana_name,
            user.email,
            user.password,
            user.phone_number,
            UserCategoryDto::from(user.category),
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
