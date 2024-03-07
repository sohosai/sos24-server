use crate::repository::{
    firebase_user::MockFirebaseUserRepository, news::MockNewsRepository, user::MockUserRepository,
    Repositories,
};

#[derive(Default)]
pub struct MockRepositories {
    firebase_user_repository: MockFirebaseUserRepository,
    news_repository: MockNewsRepository,
    user_repository: MockUserRepository,
}

impl Repositories for MockRepositories {
    type FirebaseUserRepositoryImpl = MockFirebaseUserRepository;
    type NewsRepositoryImpl = MockNewsRepository;
    type UserRepositoryImpl = MockUserRepository;

    fn firebase_user_repository(&self) -> &Self::FirebaseUserRepositoryImpl {
        &self.firebase_user_repository
    }

    fn news_repository(&self) -> &Self::NewsRepositoryImpl {
        &self.news_repository
    }

    fn user_repository(&self) -> &Self::UserRepositoryImpl {
        &self.user_repository
    }
}
