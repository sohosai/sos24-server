use sos24_domain::entity::project::Project;
use sos24_domain::entity::{
    common::date::WithDate,
    user::{User, UserRole},
};

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
}

impl From<(WithDate<User>, Option<WithDate<Project>>)> for UserDto {
    fn from((user_entity, project_entity): (WithDate<User>, Option<WithDate<Project>>)) -> Self {
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
            role: UserRoleDto::from(user.role),
            owned_project_id: project_id.map(|id| id.to_string()),
            owned_project_title: project_title,
            created_at: user_entity.created_at,
            updated_at: user_entity.updated_at,
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

impl From<UserRole> for UserRoleDto {
    fn from(entity: UserRole) -> Self {
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
