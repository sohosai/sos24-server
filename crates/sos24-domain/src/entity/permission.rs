use bitflags::bitflags;

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
                    | Permissions::UPDATE_USER_ALL
                    | Permissions::DELETE_USER_ALL
            }
            UserRole::Committee => UserRole::General.permissions() | Permissions::READ_USER_ALL,
            UserRole::General => Permissions::READ_NEWS_ALL,
        }
    }
}

#[derive(Debug)]
pub struct PermissionDeniedError;

#[macro_export]
macro_rules! ensure {
    ($expr:expr) => {
        if !$expr {
            return Err($crate::entity::permission::PermissionDeniedError.into());
        }
    };
}
