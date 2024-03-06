pub mod news;

pub trait Repositories {
    type NewsRepositoryImpl: news::NewsRepository;

    fn news_repository(&self) -> &Self::NewsRepositoryImpl;
}
