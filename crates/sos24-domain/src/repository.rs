pub mod news;
pub mod user;

pub trait Repositories {
    type NewsRepositoryImpl: news::NewsRepository;
    type UserRepositoryImpl: user::UserRepository;

    fn news_repository(&self) -> &Self::NewsRepositoryImpl;
    fn user_repository(&self) -> &Self::UserRepositoryImpl;
}
