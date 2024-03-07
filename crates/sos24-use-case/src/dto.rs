use crate::error::Result;

pub mod news;
pub mod user;

pub trait ToEntity {
    type Entity;
    type Error: std::error::Error;
    fn into_entity(self) -> Result<Self::Entity, Self::Error>;
}

pub trait FromEntity {
    type Entity;
    fn from_entity(entity: Self::Entity) -> Self;
}
