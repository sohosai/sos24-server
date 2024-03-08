use anyhow::Context;
use futures_util::{StreamExt, TryStreamExt};
use sos24_domain::{
    entity::{
        common::date::WithDate,
        user::{
            User, UserCategory, UserEmail, UserId, UserKanaName, UserName, UserPhoneNumber,
            UserRole,
        },
    },
    repository::user::{UserRepository, UserRepositoryError},
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

    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl TryFrom<UserRow> for WithDate<User> {
    type Error = anyhow::Error;
    fn try_from(value: UserRow) -> Result<Self, Self::Error> {
        Ok(WithDate::new(
            User::new(
                UserId::new(value.id),
                UserName::new(value.name),
                UserKanaName::new(value.kana_name),
                UserEmail::try_from(value.email)?,
                UserPhoneNumber::new(value.phone_number),
                UserRole::from(value.role),
                UserCategory::from(value.category),
            ),
            value.created_at,
            value.updated_at,
            value.deleted_at,
        ))
    }
}

#[derive(Type)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRoleRow {
    Administrator,
    CommitteeOperator,
    Committee,
    General,
}

impl From<UserRoleRow> for UserRole {
    fn from(value: UserRoleRow) -> Self {
        match value {
            UserRoleRow::Administrator => UserRole::Administrator,
            UserRoleRow::CommitteeOperator => UserRole::CommitteeOperator,
            UserRoleRow::Committee => UserRole::Committee,
            UserRoleRow::General => UserRole::General,
        }
    }
}

impl From<UserRole> for UserRoleRow {
    fn from(value: UserRole) -> Self {
        match value {
            UserRole::Administrator => UserRoleRow::Administrator,
            UserRole::CommitteeOperator => UserRoleRow::CommitteeOperator,
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

#[derive(Clone)]
pub struct PgUserRepository {
    db: Postgresql,
}

impl PgUserRepository {
    pub fn new(db: Postgresql) -> Self {
        Self { db }
    }
}

impl UserRepository for PgUserRepository {
    async fn list(&self) -> Result<Vec<WithDate<User>>, UserRepositoryError> {
        let user_list = sqlx::query_as!(UserRow, r#"SELECT id, name, kana_name, email, phone_number, category AS "category: UserCategoryRow", role AS "role: UserRoleRow", created_at, updated_at, deleted_at FROM users WHERE deleted_at IS NULL"#)
            .fetch(&*self.db)
            .map(|row| WithDate::try_from(row.context("Failed to fetch user list")?))
            .try_collect()
            .await?;
        Ok(user_list)
    }

    async fn create(&self, user: User) -> Result<(), UserRepositoryError> {
        let user = user.destruct();
        let res = sqlx::query!(
          r#"INSERT INTO users (id, name, kana_name, email, phone_number, category) VALUES ($1, $2, $3, $4, $5, $6)"#,
          user.id.value(),
          user.name.value(),
          user.kana_name.value(),
          user.email.clone().value(),
          user.phone_number.clone().value(),
          UserCategoryRow::from(user.category) as UserCategoryRow,
        )
        .execute(&*self.db)
        .await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => match e.as_database_error() {
                Some(e) if e.constraint() == Some("users_email_key") => {
                    Err(UserRepositoryError::EmailAlreadyUsed(user.email))
                }
                Some(e) if e.constraint() == Some("users_phone_number_key") => Err(
                    UserRepositoryError::PhoneNumberAlreadyUsed(user.phone_number),
                ),
                _ => Err(anyhow::anyhow!("Failed to create user: {e}").into()),
            },
        }
    }

    async fn find_by_id(&self, id: UserId) -> Result<Option<WithDate<User>>, UserRepositoryError> {
        let user_row = sqlx::query_as!(
            UserRow,
            r#"SELECT id, name, kana_name, email, phone_number, category AS "category: UserCategoryRow", role AS "role: UserRoleRow", created_at, updated_at, deleted_at FROM users WHERE id = $1 AND deleted_at IS NULL"#,
            id.value(),
        )
        .fetch_optional(&*self.db)
        .await
        .context("Failed to fetch user by id")?;
        Ok(user_row.map(WithDate::try_from).transpose()?)
    }

    async fn update(&self, user: User) -> Result<(), UserRepositoryError> {
        let user = user.destruct();
        sqlx::query!(
            r#"UPDATE users SET name = $2, kana_name = $3, email = $4, phone_number = $5, category = $6, role = $7 WHERE id = $1 AND deleted_at IS NULL"#,
            user.id.value(),
            user.name.value(),
            user.kana_name.value(),
            user.email.value(),
            user.phone_number.value(),
            UserCategoryRow::from(user.category) as UserCategoryRow,
            UserRoleRow::from(user.role) as UserRoleRow,
        )
        .execute(&*self.db)
        .await
        .context("Failed to update user")?;
        Ok(())
    }

    async fn delete_by_id(&self, id: UserId) -> Result<(), UserRepositoryError> {
        sqlx::query!(
            r#"UPDATE users SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL"#,
            id.value(),
        )
        .execute(&*self.db)
        .await
        .context("Failed to delete user")?;
        Ok(())
    }
}
