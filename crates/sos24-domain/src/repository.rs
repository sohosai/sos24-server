pub mod firebase_user;
pub mod form;
pub mod form_answer;
pub mod invitation;
pub mod news;
pub mod news_attachment;
pub mod project;
pub mod user;

pub trait Repositories: Send + Sync + 'static {
    type FirebaseUserRepositoryImpl: firebase_user::FirebaseUserRepository;
    type FormRepositoryImpl: form::FormRepository;
    type FormAnswerRepositoryImpl: form_answer::FormAnswerRepository;
    type InvitationRepositoryImpl: invitation::InvitationRepository;
    type NewsRepositoryImpl: news::NewsRepository;
    type ProjectRepositoryImpl: project::ProjectRepository;
    type NewsAttachmentRepositoryImpl: news_attachment::NewsAttachmentRepository;
    type UserRepositoryImpl: user::UserRepository;

    fn firebase_user_repository(&self) -> &Self::FirebaseUserRepositoryImpl;
    fn form_repository(&self) -> &Self::FormRepositoryImpl;
    fn form_answer_repository(&self) -> &Self::FormAnswerRepositoryImpl;
    fn invitation_repository(&self) -> &Self::InvitationRepositoryImpl;
    fn news_repository(&self) -> &Self::NewsRepositoryImpl;
    fn project_repository(&self) -> &Self::ProjectRepositoryImpl;
    fn news_attachment_repository(&self) -> &Self::NewsAttachmentRepositoryImpl;
    fn user_repository(&self) -> &Self::UserRepositoryImpl;
}
