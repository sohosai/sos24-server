use crate::repository::{
    firebase_user::MockFirebaseUserRepository, news::MockNewsRepository,
    project::MockProjectRepository, user::MockUserRepository, Repositories,
};

#[derive(Default)]
pub struct MockRepositories {
    firebase_user_repository: MockFirebaseUserRepository,
    news_repository: MockNewsRepository,
    project_repository: MockProjectRepository,
    user_repository: MockUserRepository,
}

impl MockRepositories {
    pub fn firebase_user_repository_mut(&mut self) -> &mut MockFirebaseUserRepository {
        &mut self.firebase_user_repository
    }

    pub fn news_repository_mut(&mut self) -> &mut MockNewsRepository {
        &mut self.news_repository
    }

    pub fn project_repository_mut(&mut self) -> &mut MockProjectRepository {
        &mut self.project_repository
    }

    pub fn user_repository_mut(&mut self) -> &mut MockUserRepository {
        &mut self.user_repository
    }
}

impl Repositories for MockRepositories {
    type FirebaseUserRepositoryImpl = MockFirebaseUserRepository;
    type NewsRepositoryImpl = MockNewsRepository;
    type ProjectRepositoryImpl = MockProjectRepository;
    type UserRepositoryImpl = MockUserRepository;

    fn firebase_user_repository(&self) -> &Self::FirebaseUserRepositoryImpl {
        &self.firebase_user_repository
    }

    fn news_repository(&self) -> &Self::NewsRepositoryImpl {
        &self.news_repository
    }

    fn project_repository(&self) -> &Self::ProjectRepositoryImpl {
        &self.project_repository
    }

    fn user_repository(&self) -> &Self::UserRepositoryImpl {
        &self.user_repository
    }
}
