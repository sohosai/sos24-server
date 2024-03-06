#[derive(Debug, Clone)]
pub struct News {
    pub id: NewsId,
    pub title: NewsTitle,
    pub body: NewsBody,
    pub categories: NewsCategories,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl News {
    pub fn new(
        id: NewsId,
        title: NewsTitle,
        body: NewsBody,
        categories: NewsCategories,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
        deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        Self {
            id,
            title,
            body,
            categories,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewsId(uuid::Uuid);

impl NewsId {
    pub fn new(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }

    pub fn value(self) -> uuid::Uuid {
        self.0
    }
}

impl TryFrom<&str> for NewsId {
    type Error = uuid::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(uuid::Uuid::parse_str(value)?))
    }
}

impl TryFrom<String> for NewsId {
    type Error = uuid::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(uuid::Uuid::parse_str(value.as_ref())?))
    }
}

#[derive(Debug, Clone)]
pub struct NewsTitle(String);

impl NewsTitle {
    pub fn new(title: String) -> Self {
        Self(title)
    }

    pub fn value(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct NewsBody(String);

impl NewsBody {
    pub fn new(body: String) -> Self {
        Self(body)
    }

    pub fn value(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct NewsCategories(i32);

impl NewsCategories {
    pub fn new(categories: i32) -> Self {
        Self(categories)
    }

    pub fn value(self) -> i32 {
        self.0
    }
}