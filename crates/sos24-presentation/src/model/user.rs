use serde::{Deserialize, Serialize};
use sos24_use_case::dto::user::{
    CreateUserDto, UpdateUserDto, UserCategoryDto, UserDto, UserRoleDto,
};

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
pub struct UpdateUser {
    pub name: String,
    pub kana_name: String,
    pub email: String,
    pub phone_number: String,
    pub role: UserRole,
    pub category: UserCategory,
}

pub trait ConvertToUpdateUserDto {
    fn to_update_user_dto(self) -> UpdateUserDto;
}

impl ConvertToUpdateUserDto for (String, UpdateUser) {
    fn to_update_user_dto(self) -> UpdateUserDto {
        let (id, user) = self;
        UpdateUserDto::new(
            id,
            user.name,
            user.kana_name,
            user.email,
            user.phone_number,
            UserRoleDto::from(user.role),
            UserCategoryDto::from(user.category),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub kana_name: String,
    pub email: String,
    pub phone_number: String,
    pub role: UserRole,
    pub category: UserCategory,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<UserDto> for User {
    fn from(value: UserDto) -> Self {
        User {
            id: value.id,
            name: value.name,
            kana_name: value.kana_name,
            email: value.email,
            phone_number: value.phone_number,
            role: value.role.into(),
            category: value.category.into(),
            created_at: value.created_at.to_rfc3339(),
            updated_at: value.updated_at.to_rfc3339(),
            deleted_at: value.deleted_at.map(|it| it.to_rfc3339()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserTobeExport {
    id: String,
    名前: String,
    なまえ: String,
    メールアドレス: String,
    ロール: String,
    作成日時: String,
}

impl From<UserDto> for UserTobeExport {
    fn from(user: UserDto) -> Self {
        UserTobeExport {
            id: user.id,
            名前: user.name,
            なまえ: user.kana_name,
            メールアドレス: user.email,
            ロール: user.role.to_string(),
            作成日時: user.created_at.to_rfc2822(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Administrator,
    CommitteeOperator,
    Committee,
    General,
}

impl From<UserRole> for UserRoleDto {
    fn from(value: UserRole) -> Self {
        match value {
            UserRole::Administrator => UserRoleDto::Administrator,
            UserRole::CommitteeOperator => UserRoleDto::CommitteeOperator,
            UserRole::Committee => UserRoleDto::Committee,
            UserRole::General => UserRoleDto::General,
        }
    }
}



impl From<UserRoleDto> for UserRole {
    fn from(value: UserRoleDto) -> Self {
        match value {
            UserRoleDto::Administrator => UserRole::Administrator,
            UserRoleDto::CommitteeOperator => UserRole::CommitteeOperator,
            UserRoleDto::Committee => UserRole::Committee,
            UserRoleDto::General => UserRole::General,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

impl From<UserCategoryDto> for UserCategory {
    fn from(value: UserCategoryDto) -> Self {
        match value {
            UserCategoryDto::UndergraduateStudent => UserCategory::UndergraduateStudent,
            UserCategoryDto::GraduateStudent => UserCategory::GraduateStudent,
            UserCategoryDto::AcademicStaff => UserCategory::AcademicStaff,
        }
    }
}
