use getset::Getters;

use super::{
    permission::Permissions,
    user::{UserId, UserRole},
};

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct Actor {
    #[getset(get = "pub")]
    user_id: UserId,
    #[getset(get = "pub")]
    role: UserRole,
}

impl Actor {
    pub fn new(user_id: UserId, role: UserRole) -> Self {
        Self { user_id, role }
    }

    pub fn new_admin() -> Self {
        Self {
            user_id: UserId::new("admin".to_string()), // TODO
            role: UserRole::Administrator,
        }
    }

    pub fn has_permission(&self, permission: Permissions) -> bool {
        self.role().permissions().contains(permission)
    }
}
