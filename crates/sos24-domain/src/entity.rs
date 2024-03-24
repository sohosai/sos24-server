pub mod actor;
pub mod common;
pub mod file_data;
pub mod file_object;
pub mod firebase_user;
pub mod form;
pub mod form_answer;
pub mod invitation;
pub mod news;
pub mod permission;
pub mod project;
pub mod project_application_period;
pub mod user;

#[macro_export]
macro_rules! impl_value_object {
    ($name:ident($inner_typ:ty)) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name($inner_typ);

        impl $name {
            pub fn new(value: $inner_typ) -> Self {
                Self(value)
            }

            pub fn value(self) -> $inner_typ {
                self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_value_object_without_new {
    ($name:ident($inner_typ:ty)) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name($inner_typ);

        impl $name {
            pub fn value(self) -> $inner_typ {
                self.0
            }
        }
    };
}
