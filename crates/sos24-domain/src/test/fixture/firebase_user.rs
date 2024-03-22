use crate::entity::firebase_user::{
    FirebaseUserEmail, FirebaseUserId, FirebaseUserPassword, NewFirebaseUser,
};

use super::user;

pub fn id1() -> FirebaseUserId {
    FirebaseUserId::new(user::id1().value())
}

pub fn email1() -> FirebaseUserEmail {
    FirebaseUserEmail::try_from(user::email1().value()).unwrap()
}

pub fn password1() -> FirebaseUserPassword {
    FirebaseUserPassword::new("password".to_string())
}

pub fn user1() -> NewFirebaseUser {
    NewFirebaseUser::new(email1(), password1())
}
