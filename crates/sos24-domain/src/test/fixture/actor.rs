use crate::entity::{actor::Actor, user::UserRole};

use super::user;

pub fn actor1(role: UserRole) -> Actor {
    Actor::new(user::id1(), role)
}

pub fn actor2(role: UserRole) -> Actor {
    Actor::new(user::id2(), role)
}
