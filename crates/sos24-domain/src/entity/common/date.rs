#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithDate<T> {
    pub value: T,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl<T> WithDate<T> {
    pub fn new(
        value: T,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
        deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        Self {
            value,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}
