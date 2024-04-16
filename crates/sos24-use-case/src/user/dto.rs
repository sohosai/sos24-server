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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<WithDate<User>> for UserDto {
    fn from(entity: WithDate<User>) -> Self {
        let user = entity.value.destruct();
        Self {
            id: user.id.value(),
            name: user.name.value(),
            kana_name: user.kana_name.value(),
            email: user.email.value(),
            phone_number: user.phone_number.value(),
            role: UserRoleDto::from(user.role),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
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
    fn from(dto: UserRoleDto) -> Self {
        match dto {
            UserRoleDto::Administrator => UserRole::Administrator,
            UserRoleDto::CommitteeOperator => UserRole::CommitteeOperator,
            UserRoleDto::Committee => UserRole::Committee,
            UserRoleDto::General => UserRole::General,
        }
    }
}

impl From<UserRole> for UserRoleDto {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::Administrator => UserRoleDto::Administrator,
            UserRole::CommitteeOperator => UserRoleDto::CommitteeOperator,
            UserRole::Committee => UserRoleDto::Committee,
            UserRole::General => UserRoleDto::General,
        }
    }
}
