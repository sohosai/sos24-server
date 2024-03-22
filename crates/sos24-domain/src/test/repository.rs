use crate::repository::{
    firebase_user::MockFirebaseUserRepository, form::MockFormRepository,
    form_answer::MockFormAnswerRepository, invitation::MockInvitationRepository,
    news::MockNewsRepository, project::MockProjectRepository, user::MockUserRepository,
    Repositories,
};

use crate::repository::{
    firebase_user::MockFirebaseUserRepository, news::MockNewsRepository,
    news_attachment::MockNewsAttachmentRepository, user::MockUserRepository, Repositories,
};

#[derive(Default)]
pub struct MockRepositories {
    firebase_user_repository: MockFirebaseUserRepository,
    news_repository: MockNewsRepository,
    news_attachment_repository: MockNewsAttachmentRepository,
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

    type NewsAttachmentRepositoryImpl = MockNewsAttachmentRepository;

    fn user_repository(&self) -> &Self::UserRepositoryImpl {
        &self.user_repository
    }

    type FormRepositoryImpl;

    type FormAnswerRepositoryImpl;

    type InvitationRepositoryImpl;

    type ProjectRepositoryImpl;

    fn form_repository(&self) -> &Self::FormRepositoryImpl {
        todo!()
    }

    fn form_answer_repository(&self) -> &Self::FormAnswerRepositoryImpl {
        todo!()
    }

    fn invitation_repository(&self) -> &Self::InvitationRepositoryImpl {
        todo!()
    }

    fn project_repository(&self) -> &Self::ProjectRepositoryImpl {
        todo!()
    }
}

#[derive(Default)]
pub struct MockRepositories {
    firebase_user_repository: MockFirebaseUserRepository,
    form_repository: MockFormRepository,
    form_answer_repository: MockFormAnswerRepository,
    invitation_repository: MockInvitationRepository,
    news_repository: MockNewsRepository,
    project_repository: MockProjectRepository,
    user_repository: MockUserRepository,
}

impl MockRepositories {
    pub fn firebase_user_repository_mut(&mut self) -> &mut MockFirebaseUserRepository {
        &mut self.firebase_user_repository
    }

    pub fn form_repository_mut(&mut self) -> &mut MockFormRepository {
        &mut self.form_repository
    }

    pub fn form_answer_repository_mut(&mut self) -> &mut MockFormAnswerRepository {
        &mut self.form_answer_repository
    }

    pub fn invitation_repository_mut(&mut self) -> &mut MockInvitationRepository {
        &mut self.invitation_repository
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
    type FormRepositoryImpl = MockFormRepository;
    type FormAnswerRepositoryImpl = MockFormAnswerRepository;
    type InvitationRepositoryImpl = MockInvitationRepository;
    type NewsRepositoryImpl = MockNewsRepository;
    type ProjectRepositoryImpl = MockProjectRepository;
    type UserRepositoryImpl = MockUserRepository;

    fn firebase_user_repository(&self) -> &Self::FirebaseUserRepositoryImpl {
        &self.firebase_user_repository
    }

    fn form_repository(&self) -> &Self::FormRepositoryImpl {
        &self.form_repository
    }

    fn form_answer_repository(&self) -> &Self::FormAnswerRepositoryImpl {
        &self.form_answer_repository
    }

    fn invitation_repository(&self) -> &Self::InvitationRepositoryImpl {
        &self.invitation_repository
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
