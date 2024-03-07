use getset::{Getters, Setters};

use crate::impl_value_object;

use super::common::email::{Email, EmailError};

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct User {
    #[getset(get = "pub")]
    id: UserId,
    #[getset(get = "pub", set = "pub")]
    name: UserName,
    #[getset(get = "pub", set = "pub")]
    kana_name: UserKanaName,
    #[getset(get = "pub", set = "pub")]
    email: UserEmail,
    #[getset(get = "pub", set = "pub")]
    phone_number: UserPhoneNumber,
    #[getset(get = "pub", set = "pub")]
    role: UserRole,
    #[getset(get = "pub", set = "pub")]
    category: UserCategory,
}

impl User {
    pub fn new(
        id: UserId,
        name: UserName,
        kana_name: UserKanaName,
        email: UserEmail,
        phone_number: UserPhoneNumber,
        role: UserRole,
        category: UserCategory,
    ) -> Self {
        Self {
            id,
            name,
            kana_name,
            email,
            phone_number,
            role,
            category,
        }
    }

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

    pub fn destruct(self) -> DestructuredUser {
        DestructuredUser {
            id: self.id,
            name: self.name,
            kana_name: self.kana_name,
            email: self.email,
            phone_number: self.phone_number,
            role: self.role,
            category: self.category,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructuredUser {
    pub id: UserId,
    pub name: UserName,
    pub kana_name: UserKanaName,
    pub email: UserEmail,
    pub phone_number: UserPhoneNumber,
    pub role: UserRole,
    pub category: UserCategory,
}

impl_value_object!(UserId(String));
impl_value_object!(UserName(String));
impl_value_object!(UserKanaName(String));
impl_value_object!(UserPhoneNumber(String)); // ガバガバだが、電話番号が弾かれる事によってjsysの作業が増えることを鑑みて許容する

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    Administrator,
    CommitteeOperator,
    Committee,
    General,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserCategory {
    UndergraduateStudent,
    GraduateStudent,
    AcademicStaff,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
