use crate::repository::{
    file_data::MockFileDataRepository, file_object::MockFileObjectRepository,
    firebase_user::MockFirebaseUserRepository, form::MockFormRepository,
    form_answer::MockFormAnswerRepository, invitation::MockInvitationRepository,
    news::MockNewsRepository, project::MockProjectRepository, user::MockUserRepository,
    MockHealthChecker, Repositories,
};

#[derive(Default)]
pub struct MockRepositories {
    firebase_user_repository: MockFirebaseUserRepository,
    form_repository: MockFormRepository,
    form_answer_repository: MockFormAnswerRepository,
    invitation_repository: MockInvitationRepository,
    news_repository: MockNewsRepository,
    file_data_repository: MockFileDataRepository,
    file_object_repository: MockFileObjectRepository,
    project_repository: MockProjectRepository,
    user_repository: MockUserRepository,
    health_checker: MockHealthChecker,
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

    pub fn file_data_repository_mut(&mut self) -> &mut MockFileDataRepository {
        &mut self.file_data_repository
    }

    pub fn file_object_repository_mut(&mut self) -> &mut MockFileObjectRepository {
        &mut self.file_object_repository
    }

    pub fn project_repository_mut(&mut self) -> &mut MockProjectRepository {
        &mut self.project_repository
    }

    pub fn user_repository_mut(&mut self) -> &mut MockUserRepository {
        &mut self.user_repository
    }

    pub fn health_checker_mut(&mut self) -> &mut MockHealthChecker {
        &mut self.health_checker
    }
}

impl Repositories for MockRepositories {
    type FirebaseUserRepositoryImpl = MockFirebaseUserRepository;
    type FormRepositoryImpl = MockFormRepository;
    type FormAnswerRepositoryImpl = MockFormAnswerRepository;
    type InvitationRepositoryImpl = MockInvitationRepository;
    type NewsRepositoryImpl = MockNewsRepository;
    type ProjectRepositoryImpl = MockProjectRepository;
    type FileDataRepositoryImpl = MockFileDataRepository;
    type FileObjectRepositoryImpl = MockFileObjectRepository;
    type UserRepositoryImpl = MockUserRepository;
    type HealthCheckerImpl = MockHealthChecker;

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

    fn file_data_repository(&self) -> &Self::FileDataRepositoryImpl {
        &self.file_data_repository
    }

    fn file_object_repository(&self) -> &Self::FileObjectRepositoryImpl {
        &self.file_object_repository
    }

    fn user_repository(&self) -> &Self::UserRepositoryImpl {
        &self.user_repository
    }

    fn health_checker(&self) -> &Self::HealthCheckerImpl {
        &self.health_checker
    }
}
