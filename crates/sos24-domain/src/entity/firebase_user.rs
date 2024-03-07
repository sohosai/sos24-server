use getset::{Getters, Setters};

use crate::impl_value_object;

use super::common::email::{Email, EmailError};

impl_value_object!(FirebaseUserId(String));

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct NewFirebaseUser {
    #[getset(get = "pub")]
    email: FirebaseUserEmail,
    #[getset(get = "pub")]
    password: FirebaseUserPassword,
}

impl NewFirebaseUser {
    pub fn new(email: FirebaseUserEmail, password: FirebaseUserPassword) -> Self {
        Self { email, password }
    }

    pub fn destruct(self) -> DestructedNewFirebaseUser {
        DestructedNewFirebaseUser {
            email: self.email,
            password: self.password,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructedNewFirebaseUser {
    pub email: FirebaseUserEmail,
    pub password: FirebaseUserPassword,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirebaseUserEmail(Email);

impl FirebaseUserEmail {
    pub fn value(self) -> String {
        self.0.value()
    }
}

impl TryFrom<String> for FirebaseUserEmail {
    type Error = EmailError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let email = Email::try_from(value)?;
        Ok(Self(email))
    }
}

impl_value_object!(FirebaseUserPassword(String));
