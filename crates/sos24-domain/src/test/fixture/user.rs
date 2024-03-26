use crate::entity::user::{
    User, UserEmail, UserId, UserKanaName, UserName, UserPhoneNumber, UserRole,
};

pub fn id1() -> UserId {
    UserId::new("user_id1".to_string())
}

pub fn name1() -> UserName {
    UserName::new("筑波太郎1".to_string())
}

pub fn kana_name1() -> UserKanaName {
    UserKanaName::new("つくばたろう1".to_string())
}

pub fn email1() -> UserEmail {
    UserEmail::try_from("this_account_should_not_exist_1@u.tsukuba.ac.jp".to_string()).unwrap()
}

pub fn phone_number1() -> UserPhoneNumber {
    UserPhoneNumber::new("0900-000-0001".to_string())
}

pub fn id2() -> UserId {
    UserId::new("user_id2".to_string())
}

pub fn name2() -> UserName {
    UserName::new("筑波太郎2".to_string())
}

pub fn kana_name2() -> UserKanaName {
    UserKanaName::new("つくばたろう2".to_string())
}

pub fn email2() -> UserEmail {
    UserEmail::try_from("this_account_should_not_exist_2@u.tsukuba.ac.jp".to_string()).unwrap()
}

pub fn phone_number2() -> UserPhoneNumber {
    UserPhoneNumber::new("0900-000-0002".to_string())
}

pub fn user1(role: UserRole) -> User {
    User::new(
        id1(),
        name1(),
        kana_name1(),
        email1(),
        phone_number1(),
        role,
    )
}

pub fn user2(role: UserRole) -> User {
    User::new(
        id2(),
        name2(),
        kana_name2(),
        email2(),
        phone_number2(),
        role,
    )
}
