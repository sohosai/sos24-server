use crate::impl_value_object;

use super::common::email::{Email, EmailError};

#[derive(Debug, Clone, PartialEq, Eq)]
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
