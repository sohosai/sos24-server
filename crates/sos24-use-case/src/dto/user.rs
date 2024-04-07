use sos24_domain::entity::project::Project;
use sos24_domain::entity::{
    common::date::WithDate,
    user::{User, UserEmail, UserId, UserKanaName, UserName, UserPhoneNumber, UserRole},
};

use crate::interactor::user::UserUseCaseError;

use super::{FromEntity, ToEntity};

#[derive(Debug)]
pub struct CreateUserDto {
    pub name: String,
    pub kana_name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
}

impl CreateUserDto {
    pub fn new(
        name: String,
        kana_name: String,
        email: String,
        password: String,
        phone_number: String,
    ) -> Self {
        Self {
            name,
            kana_name,
            email,
            password,
            phone_number,
        }
    }
}

impl ToEntity for (String, CreateUserDto) {
    type Entity = User;
    type Error = UserUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        let (id, user) = self;
        Ok(User::new_general(
            UserId::new(id),
            UserName::new(user.name),
            UserKanaName::new(user.kana_name),
            UserEmail::try_from(user.email)?,
            UserPhoneNumber::new(user.phone_number),
        ))
    }
}

#[derive(Debug)]
pub struct UpdateUserDto {
    pub id: String,
    pub name: String,
    pub kana_name: String,
    pub email: String,
    pub phone_number: String,
    pub role: UserRoleDto,
}

impl UpdateUserDto {
    pub fn new(
        id: String,
        name: String,
        kana_name: String,
        email: String,
        phone_number: String,
        role: UserRoleDto,
    ) -> Self {
        Self {
            id,
            name,
            kana_name,
            email,
            phone_number,
            role,
        }
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
    pub owned_project_id: Option<String>,
    pub owned_project_title: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FromEntity for UserDto {
    type Entity = (WithDate<User>, Option<WithDate<Project>>);
    fn from_entity((user_entity, project_entity): Self::Entity) -> Self {
        let user = user_entity.value.destruct();
        let (project_id, project_title) = match project_entity {
            Some(project) => {
                let project = project.value.destruct();
                (Some(project.id.value()), Some(project.title.value()))
            }
            None => (None, None),
        };

        Self {
            id: user.id.value(),
            name: user.name.value(),
            kana_name: user.kana_name.value(),
            email: user.email.value(),
            phone_number: user.phone_number.value(),
            role: UserRoleDto::from_entity(user.role),
            owned_project_id: project_id.map(|id| id.to_string()),
            owned_project_title: project_title,
            created_at: user_entity.created_at,
            updated_at: user_entity.updated_at,
            deleted_at: user_entity.deleted_at,
        }
    }
}

#[derive(Debug)]
pub enum UserRoleDto {
    Administrator,
    CommitteeOperator,
    Committee,
    General,
}

impl ToEntity for UserRoleDto {
    type Entity = UserRole;
    type Error = UserUseCaseError;
    fn into_entity(self) -> Result<Self::Entity, Self::Error> {
        Ok(match self {
            UserRoleDto::Administrator => UserRole::Administrator,
            UserRoleDto::CommitteeOperator => UserRole::CommitteeOperator,
            UserRoleDto::Committee => UserRole::Committee,
            UserRoleDto::General => UserRole::General,
        })
    }
}

impl FromEntity for UserRoleDto {
    type Entity = UserRole;
    fn from_entity(entity: Self::Entity) -> Self {
        match entity {
            UserRole::Administrator => UserRoleDto::Administrator,
            UserRole::CommitteeOperator => UserRoleDto::CommitteeOperator,
            UserRole::Committee => UserRoleDto::Committee,
            UserRole::General => UserRoleDto::General,
        }
    }
}

impl std::fmt::Display for UserRoleDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRoleDto::Administrator => write!(f, "管理者"),
            UserRoleDto::CommitteeOperator => write!(f, "実委人(管理者)"),
            UserRoleDto::Committee => write!(f, "実委人"),
            UserRoleDto::General => write!(f, "一般"),
        }
    }
}
