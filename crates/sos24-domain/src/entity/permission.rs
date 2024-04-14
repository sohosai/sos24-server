use bitflags::bitflags;
use thiserror::Error;

use super::user::UserRole;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permissions(u32);

bitflags! {
    impl Permissions: u32 {
      const CREATE_NEWS = 1 << 0;
      const READ_NEWS_ALL = 1 << 1;
      const UPDATE_NEWS_ALL = 1 << 2;
      const DELETE_NEWS_ALL = 1 << 3;

      const READ_USER_ALL = 1 << 4;
      const UPDATE_USER_ALL = 1 << 5;
      const DELETE_USER_ALL = 1 << 6;

      const CREATE_PROJECT = 1 << 7;
      const READ_PROJECT_ALL = 1 << 8;
      const UPDATE_PROJECT_ALL = 1 << 9;
      const DELETE_PROJECT_ALL = 1 << 10;

      const CREATE_FORM = 1 << 11;
      const READ_FORM_ALL = 1 << 12;
      const UPDATE_FORM_ALL = 1 << 13;
      const DELETE_FORM_ALL = 1 << 14;

      const CREATE_INVITATION = 1 << 15;
      const CREATE_INVITATION_ANYTIME = 1 << 16;
      const READ_INVITATION_ALL = 1 << 17;
      const UPDATE_INVITATION_ALL = 1 << 18;
      const DELETE_INVITATION_ALL = 1 << 19;

      const CREATE_FORM_ANSWER = 1 << 20;
      const READ_FORM_ANSWER_ALL = 1 << 21;
      const UPDATE_FORM_ANSWER_ALL = 1 << 22;

      const CREATE_FILE_PRIVATE = 1 << 23;
      const CREATE_FILE_PUBLIC = 1 << 24;
      const READ_FILE_ALL = 1 << 25;
      const DELETE_FILE_ALL = 1 << 26;

      const CREATE_PROJECT_ANYTIME = 1 << 27;
    }
}

impl UserRole {
    pub fn permissions(&self) -> Permissions {
        match self {
            UserRole::Administrator => Permissions::all(),
            UserRole::CommitteeOperator => {
                UserRole::Committee.permissions()
                    | Permissions::CREATE_NEWS
                    | Permissions::UPDATE_NEWS_ALL
                    | Permissions::DELETE_NEWS_ALL
                    | Permissions::READ_USER_ALL
                    | Permissions::UPDATE_USER_ALL
                    | Permissions::DELETE_USER_ALL
                    | Permissions::UPDATE_PROJECT_ALL
                    | Permissions::DELETE_PROJECT_ALL
                    | Permissions::CREATE_FORM
                    | Permissions::UPDATE_FORM_ALL
                    | Permissions::DELETE_FORM_ALL
                    | Permissions::UPDATE_INVITATION_ALL
                    | Permissions::DELETE_INVITATION_ALL
                    | Permissions::CREATE_INVITATION_ANYTIME
                    | Permissions::UPDATE_FORM_ANSWER_ALL
                    | Permissions::CREATE_FILE_PUBLIC
                    | Permissions::DELETE_FILE_ALL
                    | Permissions::CREATE_PROJECT_ANYTIME
            }
            UserRole::Committee => {
                UserRole::General.permissions()
                    | Permissions::READ_PROJECT_ALL
                    | Permissions::READ_INVITATION_ALL
                    | Permissions::READ_FORM_ANSWER_ALL
                    | Permissions::READ_FILE_ALL
            }
            UserRole::General => {
                Permissions::READ_NEWS_ALL
                    | Permissions::CREATE_PROJECT
                    | Permissions::CREATE_INVITATION
                    | Permissions::READ_FORM_ALL
                    | Permissions::CREATE_FORM_ANSWER
                    | Permissions::CREATE_FILE_PRIVATE
            }
        }
    }
}

#[derive(Debug, Error)]
#[error("Permission denied")]
pub struct PermissionDeniedError;

#[macro_export]
macro_rules! ensure {
    ($expr:expr) => {
        if !$expr {
            return Err($crate::entity::permission::PermissionDeniedError.into());
        }
    };
}
