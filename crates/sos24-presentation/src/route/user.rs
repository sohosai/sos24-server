use serde::{Deserialize, Serialize};
use sos24_use_case::user::{
    dto::{UserDto, UserRoleDto},
    use_case::find_by_id::UserWithProjectDto,
};

pub mod delete_by_id;
pub mod export;
pub mod get;
pub mod get_by_id;
pub mod get_me;
pub mod post;
pub mod put_by_id;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub kana_name: String,
    pub email: String,
    pub phone_number: String,
    pub role: UserRole,
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
            created_at: value.created_at.to_rfc3339(),
            updated_at: value.updated_at.to_rfc3339(),
            deleted_at: value.deleted_at.map(|it| it.to_rfc3339()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserWithProject {
    pub id: String,
    pub name: String,
    pub kana_name: String,
    pub email: String,
    pub phone_number: String,
    pub role: UserRole,
    pub owned_project_id: Option<String>,
    pub owned_project_title: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<UserWithProjectDto> for UserWithProject {
    fn from(value: UserWithProjectDto) -> Self {
        UserWithProject {
            id: value.id,
            name: value.name,
            kana_name: value.kana_name,
            email: value.email,
            phone_number: value.phone_number,
            role: value.role.into(),
            owned_project_id: value.owned_project_id,
            owned_project_title: value.owned_project_title,
            created_at: value.created_at.to_rfc3339(),
            updated_at: value.updated_at.to_rfc3339(),
            deleted_at: value.deleted_at.map(|it| it.to_rfc3339()),
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

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Administrator => write!(f, "管理者"),
            UserRole::CommitteeOperator => write!(f, "実委人(管理者)"),
            UserRole::Committee => write!(f, "実委人"),
            UserRole::General => write!(f, "一般"),
        }
    }
}
