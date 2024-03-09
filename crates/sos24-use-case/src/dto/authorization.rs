use sos24_domain::entity::{actor::Actor, permission::PermissionDeniedError};

pub mod news;
pub mod user;

pub struct PermissionGate<T>(T);

impl<T> PermissionGate<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }
}

pub trait PermissionGateExt<A, T> {
    fn new(inner: T) -> Self;
    fn for_create(self, actor: &Actor) -> Result<T, PermissionDeniedError>;
    fn for_read(self, actor: &Actor) -> Result<T, PermissionDeniedError>;
    fn for_update(self, actor: &Actor) -> Result<T, PermissionDeniedError>;
    fn for_delete(self, actor: &Actor) -> Result<T, PermissionDeniedError>;
}
