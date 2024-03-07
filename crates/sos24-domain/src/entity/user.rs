use super::common::email::{Email, EmailError};

#[derive(Debug)]
pub struct User {
    pub id: UserId,

    pub name: UserName,
    pub kana_name: UserKanaName,

    pub email: UserEmail,
    pub phone_number: UserPhoneNumber,
    pub role: UserRole,
    pub category: UserCategory,
}

impl User {
    pub fn new_general(
        id: UserId,
        name: UserName,
        kana_name: UserKanaName,
        email: UserEmail,
        phone_number: UserPhoneNumber,
        category: UserCategory,
    ) -> Self {
        Self {
            id,
            name,
            kana_name,
            email,
            phone_number,
            role: UserRole::General,
            category,
        }
    }
}

#[derive(Debug)]
pub struct UserId(String);

impl UserId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn value(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn value(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub struct UserKanaName(String);

impl UserKanaName {
    pub fn new(kana_name: String) -> Self {
        Self(kana_name)
    }

    pub fn value(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub struct UserPhoneNumber(String);

impl UserPhoneNumber {
    // ガバガバだが、電話番号が弾かれる事によってjsysの作業が増えることを鑑みて許容する
    pub fn new(phone_number: String) -> Self {
        Self(phone_number)
    }

    pub fn value(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub enum UserRole {
    Administrator,
    ComitteeOperator,
    Committee,
    General,
}

#[derive(Debug)]
pub enum UserCategory {
    UndergraduateStudent,
    GraduateStudent,
    AcademicStaff,
}

#[derive(Debug)]
pub struct UserEmail(Email);

impl UserEmail {
    pub fn value(self) -> String {
        self.0.value()
    }
}

impl TryFrom<String> for UserEmail {
    type Error = EmailError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let email = Email::try_from(value)?;
        Ok(Self(email))
    }
}
