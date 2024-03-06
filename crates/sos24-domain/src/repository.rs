pub mod news;

pub trait Repositories {
    type NewsRepositoryImpl: NewsRepository;

    fn news_repository(&self) -> &Self::NewsRepositoryImpl;
}
