pub mod firebase_user;
pub mod news;
pub mod project;
pub mod user;

pub trait Repositories: Send + Sync + 'static {
    type FirebaseUserRepositoryImpl: firebase_user::FirebaseUserRepository;
    type NewsRepositoryImpl: news::NewsRepository;
    type ProjectRepositoryImpl: project::ProjectRepository;
    type UserRepositoryImpl: user::UserRepository;

    fn firebase_user_repository(&self) -> &Self::FirebaseUserRepositoryImpl;
    fn news_repository(&self) -> &Self::NewsRepositoryImpl;
    fn project_repository(&self) -> &Self::ProjectRepositoryImpl;
    fn user_repository(&self) -> &Self::UserRepositoryImpl;
}
