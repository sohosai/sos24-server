use sos24_domain::{
    ensure,
    entity::{
        actor::Actor,
        news::{News, NewsId},
        permission::{PermissionDeniedError, Permissions},
    },
};

use crate::dto::news::NewsDto;

use super::{PermissionGate, PermissionGateExt};

trait NewsTrait {}
impl NewsTrait for NewsId {}
impl NewsTrait for News {}
impl NewsTrait for NewsDto {}

impl<T: NewsTrait> PermissionGateExt<News, T> for PermissionGate<T> {
    fn new(inner: T) -> Self {
        Self(inner)
    }

    fn for_create(self, actor: &Actor) -> Result<T, PermissionDeniedError> {
        ensure!(actor.has_permission(Permissions::CREATE_NEWS));
        Ok(self.0)
    }

    fn for_read(self, actor: &Actor) -> Result<T, PermissionDeniedError> {
        ensure!(actor.has_permission(Permissions::READ_NEWS_ALL));
        Ok(self.0)
    }

    fn for_update(self, actor: &Actor) -> Result<T, PermissionDeniedError> {
        ensure!(actor.has_permission(Permissions::UPDATE_NEWS_ALL));
        Ok(self.0)
    }

    fn for_delete(self, actor: &Actor) -> Result<T, PermissionDeniedError> {
        ensure!(actor.has_permission(Permissions::DELETE_NEWS_ALL));
        Ok(self.0)
    }
}
