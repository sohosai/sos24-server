pub mod firebase_user;
pub mod news;
pub mod news_attachment;
pub mod user;

pub trait Repositories: Send + Sync + 'static {
    type FirebaseUserRepositoryImpl: firebase_user::FirebaseUserRepository;
    type NewsRepositoryImpl: news::NewsRepository;
    type NewsAttachmentRepositoryImpl: news_attachment::NewsAttachmentRepository;
    type UserRepositoryImpl: user::UserRepository;

    fn firebase_user_repository(&self) -> &Self::FirebaseUserRepositoryImpl;
    fn news_repository(&self) -> &Self::NewsRepositoryImpl;
    fn news_attachment_repository(&self) -> &Self::NewsAttachmentRepositoryImpl;
    fn user_repository(&self) -> &Self::UserRepositoryImpl;
}
