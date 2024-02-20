use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub(crate) struct User {
    id: String,

    name: String,
    kana_name: String,

    email: String,
    phone_number: String,
    role: UserRole,
    category: UserCategory,

    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
enum UserRole {
    General,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "user_category", rename_all = "snake_case")]
enum UserCategory {
    UndergraduateStudent,
    GraduateStudent,
    AcademicStaff,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CreateUserInput {
    pub(crate) id: String,

    pub(crate) name: String,
    pub(crate) kana_name: String,

    pub(crate) email: String,
    pub(crate) phone_number: String,

    pub(crate) category: String,
}

pub(crate) async fn create_user(pool: &PgPool, input: CreateUserInput) -> Result<User> {
    let created_user = sqlx::query_as::<_, User>(
        r#"insert into users (id, name, kana_name, email, phone_number, category) values ($1, $2, $3, $4, $5, $6) returning *"#,
    )
    .bind(&input.id)
    .bind(&input.name)
    .bind(&input.kana_name)
    .bind(&input.email)
    .bind(&input.phone_number)
    .bind(UserCategory::UndergraduateStudent as UserCategory)
    .fetch_one(pool)
    .await?;

    Ok(created_user)
}
