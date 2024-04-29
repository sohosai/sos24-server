use crate::entity::common::date::WithDate;

pub fn created_at() -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::default()
}

pub fn updated_at() -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::default()
}

pub fn not_deleted() -> Option<chrono::DateTime<chrono::Utc>> {
    None
}

pub fn deleted() -> Option<chrono::DateTime<chrono::Utc>> {
    Some(chrono::DateTime::default())
}

pub fn with<T>(inner: T) -> WithDate<T> {
    WithDate {
        value: inner,
        created_at: created_at(),
        updated_at: updated_at(),
    }
}

pub fn with_deleted<T>(inner: T) -> WithDate<T> {
    WithDate {
        value: inner,
        created_at: created_at(),
        updated_at: updated_at(),
    }
}
