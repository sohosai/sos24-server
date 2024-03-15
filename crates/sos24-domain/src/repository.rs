pub mod firebase_user;
pub mod form;
pub mod invitation;
pub mod news;
pub mod project;
pub mod user;

pub trait Repositories: Send + Sync + 'static {
    type FirebaseUserRepositoryImpl: firebase_user::FirebaseUserRepository;
    type FormRepositoryImpl: form::FormRepository;
    type InvitationRepositoryImpl: invitation::InvitationRepository;
    type NewsRepositoryImpl: news::NewsRepository;
    type ProjectRepositoryImpl: project::ProjectRepository;
    type UserRepositoryImpl: user::UserRepository;

    fn firebase_user_repository(&self) -> &Self::FirebaseUserRepositoryImpl;
    fn form_repository(&self) -> &Self::FormRepositoryImpl;
    fn invitation_repository(&self) -> &Self::InvitationRepositoryImpl;
    fn news_repository(&self) -> &Self::NewsRepositoryImpl;
    fn project_repository(&self) -> &Self::ProjectRepositoryImpl;
    fn user_repository(&self) -> &Self::UserRepositoryImpl;
}
