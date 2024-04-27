pub mod file;
pub mod form;
pub mod form_answer;
pub mod invitation;
pub mod news;
pub mod project;
pub mod project_application_period;
pub mod shared;
pub mod user;

pub trait FromEntity {
    type Entity;
    fn from_entity(entity: Self::Entity) -> Self;
}
