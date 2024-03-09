use sos24_domain::{
    ensure,
    entity::{
        actor::Actor,
        permission::{PermissionDeniedError, Permissions},
        user::{User, UserId},
    },
};

use crate::dto::user::UserDto;

use super::{PermissionGate, PermissionGateExt};

trait UserTrait {}
impl UserTrait for UserId {}
impl UserTrait for User {}
impl UserTrait for UserDto {}

impl<T: UserTrait> PermissionGateExt<User, T> for PermissionGate<T> {
    fn new(inner: T) -> Self {
        Self(inner)
    }

    fn for_create(self, _: &Actor) -> Result<T, PermissionDeniedError> {
        panic!();
    }

    fn for_read(self, actor: &Actor) -> Result<T, PermissionDeniedError> {
        ensure!(actor.has_permission(Permissions::READ_USER_ALL));
        Ok(self.0)
    }

    fn for_update(self, actor: &Actor) -> Result<T, PermissionDeniedError> {
        ensure!(actor.has_permission(Permissions::UPDATE_USER_ALL));
        Ok(self.0)
    }

    fn for_delete(self, actor: &Actor) -> Result<T, PermissionDeniedError> {
        ensure!(actor.has_permission(Permissions::DELETE_USER_ALL));
        Ok(self.0)
    }
}
