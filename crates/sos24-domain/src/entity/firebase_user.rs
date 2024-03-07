use super::common::email::{Email, EmailError};

#[derive(Debug, Clone)]
pub struct FirebaseUserId(String);

impl FirebaseUserId {
    pub fn new(uid: String) -> Self {
        Self(uid)
    }

    pub fn value(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub struct NewFirebaseUser {
    pub email: FirebaseUserEmail,
    pub password: FirebaseUserPassword,
}

impl NewFirebaseUser {
    pub fn new(email: FirebaseUserEmail, password: FirebaseUserPassword) -> Self {
        Self { email, password }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct FirebaseUserPassword(String);

impl FirebaseUserPassword {
    pub fn new(password: String) -> Self {
        Self(password)
    }

    pub fn value(self) -> String {
        self.0
    }
}
