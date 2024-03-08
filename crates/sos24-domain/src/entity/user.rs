use getset::Getters;

use crate::{ensure, impl_value_object};

use super::{
    actor::Actor,
    common::email::{Email, EmailError},
    permission::{PermissionDeniedError, Permissions},
};

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct User {
    #[getset(get = "pub")]
    id: UserId,
    #[getset(get = "pub")]
    name: UserName,
    #[getset(get = "pub")]
    kana_name: UserKanaName,
    #[getset(get = "pub")]
    email: UserEmail,
    #[getset(get = "pub")]
    phone_number: UserPhoneNumber,
    #[getset(get = "pub")]
    role: UserRole,
    #[getset(get = "pub")]
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

impl User {
    pub fn is_visible_to(&self, actor: &Actor) -> bool {
        let a = actor.user_id() == self.id();
        a || actor.has_permission(Permissions::READ_USER_ALL)
    }

    fn is_updatable_by(&self, actor: &Actor) -> bool {
        actor.user_id() == self.id() || actor.has_permission(Permissions::UPDATE_USER_ALL)
    }

    pub fn set_name(&mut self, actor: &Actor, name: UserName) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.name = name;
        Ok(())
    }

    pub fn set_kana_name(
        &mut self,
        actor: &Actor,
        kana_name: UserKanaName,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.kana_name = kana_name;
        Ok(())
    }

    pub fn set_email(
        &mut self,
        actor: &Actor,
        email: UserEmail,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.email = email;
        Ok(())
    }

    pub fn set_phone_number(
        &mut self,
        actor: &Actor,
        phone_number: UserPhoneNumber,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.phone_number = phone_number;
        Ok(())
    }

    pub fn set_role(&mut self, actor: &Actor, role: UserRole) -> Result<(), PermissionDeniedError> {
        ensure!(actor.has_permission(Permissions::UPDATE_USER_ALL)); // TODO
        self.role = role;
        Ok(())
    }

    pub fn set_category(
        &mut self,
        actor: &Actor,
        category: UserCategory,
    ) -> Result<(), PermissionDeniedError> {
        ensure!(self.is_updatable_by(actor));
        self.category = category;
        Ok(())
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
