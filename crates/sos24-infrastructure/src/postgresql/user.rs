use anyhow::Context;
use sos24_domain::{
    entity::user::{
        User, UserCategory, UserEmail, UserId, UserKanaName, UserName, UserPhoneNumber, UserRole,
    },
    repository::user::UserRepository,
};
use sqlx::prelude::*;

use super::Postgresql;

#[derive(FromRow)]
pub struct UserRow {
    id: String,

    name: String,
    kana_name: String,

    email: String,
    phone_number: String,
    role: UserRoleRow,
    category: UserCategoryRow,
}

impl TryFrom<UserRow> for User {
    type Error = anyhow::Error;
    fn try_from(value: UserRow) -> Result<Self, Self::Error> {
        Ok(User {
            id: UserId::new(value.id),
            name: UserName::new(value.name),
            kana_name: UserKanaName::new(value.kana_name),
            email: UserEmail::try_from(value.email)?,
            phone_number: UserPhoneNumber::new(value.phone_number),
            role: UserRole::from(value.role),
            category: UserCategory::from(value.category),
        })
    }
}

#[derive(Type)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRoleRow {
    Administrator,
    ComitteeOperator,
    Committee,
    General,
}

impl From<UserRoleRow> for UserRole {
    fn from(value: UserRoleRow) -> Self {
        match value {
            UserRoleRow::Administrator => UserRole::Administrator,
            UserRoleRow::ComitteeOperator => UserRole::ComitteeOperator,
            UserRoleRow::Committee => UserRole::Committee,
            UserRoleRow::General => UserRole::General,
        }
    }
}

impl From<UserRole> for UserRoleRow {
    fn from(value: UserRole) -> Self {
        match value {
            UserRole::Administrator => UserRoleRow::Administrator,
            UserRole::ComitteeOperator => UserRoleRow::ComitteeOperator,
            UserRole::Committee => UserRoleRow::Committee,
            UserRole::General => UserRoleRow::General,
        }
    }
}

#[derive(Type)]
#[sqlx(type_name = "user_category", rename_all = "snake_case")]
pub enum UserCategoryRow {
    UndergraduateStudent,
    GraduateStudent,
    AcademicStaff,
}

impl From<UserCategoryRow> for UserCategory {
    fn from(value: UserCategoryRow) -> Self {
        match value {
            UserCategoryRow::UndergraduateStudent => UserCategory::UndergraduateStudent,
            UserCategoryRow::GraduateStudent => UserCategory::GraduateStudent,
            UserCategoryRow::AcademicStaff => UserCategory::AcademicStaff,
        }
    }
}

impl From<UserCategory> for UserCategoryRow {
    fn from(value: UserCategory) -> Self {
        match value {
            UserCategory::UndergraduateStudent => UserCategoryRow::UndergraduateStudent,
            UserCategory::GraduateStudent => UserCategoryRow::GraduateStudent,
            UserCategory::AcademicStaff => UserCategoryRow::AcademicStaff,
        }
    }
}

pub struct PgUserRepository {
    db: Postgresql,
}

impl PgUserRepository {
    pub fn new(db: Postgresql) -> Self {
        Self { db }
    }
}

impl UserRepository for PgUserRepository {
    async fn create(&self, user: User) -> anyhow::Result<()> {
        sqlx::query!(
          r#"INSERT INTO users (id, name, kana_name, email, phone_number, category) VALUES ($1, $2, $3, $4, $5, $6)"#,
          user.id.value(),
          user.name.value(),
          user.kana_name.value(),
          user.email.value(),
          user.phone_number.value(),
          UserCategoryRow::from(user.category) as UserCategoryRow,
        )
        .execute(&*self.db)
        .await
        .context("Failed to create user")?;

        Ok(())
    }
}
