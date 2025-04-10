use chrono_tz::Asia::Tokyo;
use serde::{Deserialize, Serialize};

use sos24_use_case::user::{
    dto::{UserDto, UserRoleDto},
    interactor::{create::CreateUserCommand, update::UpdateUserCommand},
};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUser {
    pub name: String,
    pub kana_name: String,
    pub email: String,
    #[schema(format = "password")]
    pub password: String,
    pub phone_number: String,
}

impl From<CreateUser> for CreateUserCommand {
    fn from(value: CreateUser) -> Self {
        CreateUserCommand {
            name: value.name,
            kana_name: value.kana_name,
            email: value.email,
            password: value.password,
            phone_number: value.phone_number,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedUser {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUser {
    pub name: String,
    pub kana_name: String,
    pub email: String,
    pub phone_number: String,
    pub role: UserRole,
}

pub trait ConvertToUpdateUserDto {
    fn to_update_user_dto(self) -> UpdateUserCommand;
}

impl ConvertToUpdateUserDto for (String, UpdateUser) {
    fn to_update_user_dto(self) -> UpdateUserCommand {
        let (id, user) = self;
        UpdateUserCommand {
            id,
            name: user.name,
            kana_name: user.kana_name,
            email: user.email,
            phone_number: user.phone_number,
            role: UserRoleDto::from(user.role),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: String,
    pub name: String,
    pub kana_name: String,
    pub email: String,
    pub phone_number: String,
    pub role: UserRole,
    #[schema(format = "uuid")]
    pub owned_project_id: Option<String>,
    pub owned_project_title: Option<String>,
    #[schema(format = "date-time")]
    pub created_at: String,
    #[schema(format = "date-time")]
    pub updated_at: String,
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
            owned_project_id: value.owned_project_id,
            owned_project_title: value.owned_project_title,
            created_at: value.created_at.to_rfc3339(),
            updated_at: value.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserSummary {
    id: String,
    name: String,
    email: String,
    role: UserRole,
}

impl From<UserDto> for UserSummary {
    fn from(value: UserDto) -> Self {
        UserSummary {
            id: value.id,
            name: value.name,
            email: value.email,
            role: value.role.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserTobeExported {
    #[serde(rename(serialize = "ID"))]
    id: String,
    #[serde(rename(serialize = "名前"))]
    name: String,
    #[serde(rename(serialize = "なまえ"))]
    kana_name: String,
    #[serde(rename(serialize = "メールアドレス"))]
    email: String,
    #[serde(rename(serialize = "権限"))]
    role: String,
    #[serde(rename(serialize = "作成日時"))]
    created_at: String,
}

impl From<UserDto> for UserTobeExported {
    fn from(user: UserDto) -> Self {
        UserTobeExported {
            id: user.id,
            name: user.name,
            kana_name: user.kana_name,
            email: user.email,
            role: user.role.to_string(),
            created_at: user
                .created_at
                .with_timezone(&Tokyo)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Administrator,
    CommitteeOperator,
    CommitteeEditor,
    CommitteeDrafter,
    CommitteeViewer,
    General,
}

impl From<UserRole> for UserRoleDto {
    fn from(value: UserRole) -> Self {
        match value {
            UserRole::Administrator => UserRoleDto::Administrator,
            UserRole::CommitteeOperator => UserRoleDto::CommitteeOperator,
            UserRole::CommitteeEditor => UserRoleDto::CommitteeEditor,
            UserRole::CommitteeDrafter => UserRoleDto::CommitteeDrafter,
            UserRole::CommitteeViewer => UserRoleDto::CommitteeViewer,
            UserRole::General => UserRoleDto::General,
        }
    }
}

impl From<UserRoleDto> for UserRole {
    fn from(value: UserRoleDto) -> Self {
        match value {
            UserRoleDto::Administrator => UserRole::Administrator,
            UserRoleDto::CommitteeOperator => UserRole::CommitteeOperator,
            UserRoleDto::CommitteeEditor => UserRole::CommitteeEditor,
            UserRoleDto::CommitteeDrafter => UserRole::CommitteeDrafter,
            UserRoleDto::CommitteeViewer => UserRole::CommitteeViewer,
            UserRoleDto::General => UserRole::General,
        }
    }
}
