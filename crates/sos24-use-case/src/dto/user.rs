use sos24_domain::entity::{
    common::date::WithDate,
    user::{
        User, UserCategory, UserEmail, UserId, UserKanaName, UserName, UserPhoneNumber, UserRole,
    },
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
    type Error = UserUseCaseError;
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
pub struct UpdateUserDto {
    pub id: String,
    pub name: String,
    pub kana_name: String,
    pub email: String,
    pub phone_number: String,
    pub role: UserRoleDto,
    pub category: UserCategoryDto,
}

impl UpdateUserDto {
    pub fn new(
        id: String,
        name: String,
        kana_name: String,
        email: String,
        phone_number: String,
        role: UserRoleDto,
        category: UserCategoryDto,
    ) -> Self {
        Self {
            id,
            name,
            kana_name,
            email,
            phone_number,
            role,
            category,
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
    pub category: UserCategoryDto,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FromEntity for UserDto {
    type Entity = WithDate<User>;
    fn from_entity(entity: Self::Entity) -> Self {
        let user = entity.value.destruct();
        Self {
            id: user.id.value(),
            name: user.name.value(),
            kana_name: user.kana_name.value(),
            email: user.email.value(),
            phone_number: user.phone_number.value(),
            role: UserRoleDto::from_entity(user.role),
            category: UserCategoryDto::from_entity(user.category),
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
#[derive(Debug)]
pub enum UserCategoryDto {
    UndergraduateStudent,
    GraduateStudent,
    AcademicStaff,
}

impl ToEntity for UserCategoryDto {
    type Entity = UserCategory;
    type Error = UserUseCaseError;
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
