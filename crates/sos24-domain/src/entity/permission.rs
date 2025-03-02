use bitflags::bitflags;
use thiserror::Error;

use super::user::UserRole;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permissions(u64);

bitflags! {
    impl Permissions: u64 {
      // projects
      const CREATE_PROJECT = 1 << 0;
      const CREATE_PROJECT_ANYTIME = 1 << 1;
      const READ_PROJECT_ALL = 1 << 2;
      const UPDATE_PROJECT_ALL = 1 << 3;
      const DELETE_PROJECT_ALL = 1 << 4;

      // users
      const READ_USER_ALL = 1 << 5;
      const UPDATE_USER_ALL = 1 << 6;
      const DELETE_USER_ALL = 1 << 7;

      // news
      const CREATE_NEWS = 1 << 8;
      const READ_NEWS_ALL = 1 << 9;
      const UPDATE_NEWS_ALL = 1 << 10;
      const DELETE_NEWS_ALL = 1 << 11;

      const CREATE_SCHEDULED_NEWS = 1 << 12;
      const READ_SCHEDULED_NEWS_ALL = 1 << 13;
      const UPDATE_SCHEDULED_NEWS_ALL = 1 << 14;
      const DELETE_SCHEDULED_NEWS_ALL= 1 << 15;

      const CREATE_DRAFT_NEWS = 1 << 16;
      const READ_DRAFT_NEWS_ALL = 1 << 17;
      const UPDATE_DRAFT_NEWS_ALL = 1 << 18;
      const DELETE_DRAFT_NEWS_ALL = 1 << 19;

      // forms
      const CREATE_FORM = 1 << 20;
      const READ_FORM_ALL = 1 << 21;
      const UPDATE_FORM_ALL = 1 << 22;
      const UPDATE_FORM_ALL_ANSWERED = 1 << 23;
      const DELETE_FORM_ALL = 1 << 24;

      const CREATE_SCHEDULED_FORM = 1 << 25;
      const READ_SCHEDULED_FORM_ALL = 1 << 26;
      const UPDATE_SCHEDULED_FORM_ALL = 1 << 27;
      const DELETE_SCHEDULED_FORM_ALL = 1 << 28;

      const CREATE_DRAFT_FORM = 1 << 29;
      const READ_DRAFT_FORM_ALL = 1 << 30;
      const UPDATE_DRAFT_FORM_ALL = 1 << 31;
      const DELETE_DRAFT_FORM_ALL = 1 << 32;

      // form answers
      const CREATE_FORM_ANSWER = 1 << 33;
      const READ_FORM_ANSWER_ALL = 1 << 34;
      const UPDATE_FORM_ANSWER_ALL = 1 << 35;
      const UPDATE_FORM_ANSWER_ANYTIME = 1 << 36;

      // invitations
      const CREATE_INVITATION = 1 << 37;
      const CREATE_INVITATION_ANYTIME = 1 << 38;
      const READ_INVITATION_ALL = 1 << 39;
      const UPDATE_INVITATION_ALL = 1 << 40;
      const DELETE_INVITATION_ALL = 1 << 41;

      // files
      const CREATE_FILE_PRIVATE = 1 << 42;
      const CREATE_FILE_PUBLIC = 1 << 43;
      const READ_FILE_ALL = 1 << 44;
      const DELETE_FILE_ALL = 1 << 45;

    }
}

impl UserRole {
    pub fn permissions(&self) -> Permissions {
        match self {
            UserRole::Administrator => Permissions::all(),
            UserRole::CommitteeOperator => {
                UserRole::CommitteeEditor.permissions()
                    | Permissions::DELETE_PROJECT_ALL
                    | Permissions::DELETE_USER_ALL
                    | Permissions::DELETE_NEWS_ALL
                    | Permissions::DELETE_SCHEDULED_NEWS_ALL
                    | Permissions::DELETE_DRAFT_NEWS_ALL
                    | Permissions::DELETE_FORM_ALL
                    | Permissions::DELETE_SCHEDULED_FORM_ALL
                    | Permissions::DELETE_DRAFT_FORM_ALL
                    | Permissions::DELETE_INVITATION_ALL
                    | Permissions::DELETE_FILE_ALL
            }
            UserRole::CommitteeEditor => {
                UserRole::CommitteeDrafter.permissions()
                    | Permissions::CREATE_PROJECT_ANYTIME
                    | Permissions::UPDATE_PROJECT_ALL
                    | Permissions::READ_USER_ALL
                    | Permissions::UPDATE_USER_ALL
                    | Permissions::CREATE_NEWS
                    | Permissions::UPDATE_NEWS_ALL
                    | Permissions::CREATE_SCHEDULED_NEWS
                    | Permissions::UPDATE_SCHEDULED_NEWS_ALL
                    | Permissions::UPDATE_DRAFT_NEWS_ALL
                    | Permissions::CREATE_FORM
                    | Permissions::UPDATE_FORM_ALL
                    | Permissions::CREATE_SCHEDULED_FORM
                    | Permissions::UPDATE_SCHEDULED_FORM_ALL
                    | Permissions::UPDATE_DRAFT_FORM_ALL
                    | Permissions::UPDATE_FORM_ANSWER_ALL
                    | Permissions::UPDATE_FORM_ANSWER_ANYTIME
                    | Permissions::CREATE_INVITATION_ANYTIME
                    | Permissions::UPDATE_INVITATION_ALL
                    | Permissions::CREATE_FILE_PUBLIC
            }
            UserRole::CommitteeDrafter => {
                UserRole::CommitteeViewer.permissions()
                    | Permissions::READ_SCHEDULED_NEWS_ALL
                    | Permissions::CREATE_DRAFT_NEWS
                    | Permissions::READ_DRAFT_NEWS_ALL
                    | Permissions::READ_SCHEDULED_FORM_ALL
                    | Permissions::CREATE_DRAFT_FORM
                    | Permissions::READ_DRAFT_FORM_ALL
            }

            UserRole::CommitteeViewer => {
                UserRole::General.permissions()
                    | Permissions::READ_PROJECT_ALL
                    | Permissions::READ_FORM_ANSWER_ALL
                    | Permissions::READ_INVITATION_ALL
                    | Permissions::READ_FILE_ALL
            }
            UserRole::General => {
                Permissions::READ_NEWS_ALL
                    | Permissions::CREATE_PROJECT
                    | Permissions::READ_FORM_ALL
                    | Permissions::CREATE_FORM_ANSWER
                    | Permissions::CREATE_INVITATION
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
