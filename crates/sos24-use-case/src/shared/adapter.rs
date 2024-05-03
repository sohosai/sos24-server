use self::{email::MockEmailSender, notification::MockNotifier};

pub mod email;
pub mod notification;

pub trait Adapters: Send + Sync + 'static {
    type EmailSenderImpl: email::EmailSender;
    type NotifierImpl: notification::Notifier;

    fn email_sender(&self) -> &Self::EmailSenderImpl;
    fn notifier(&self) -> &Self::NotifierImpl;
}

#[derive(Default)]
pub struct MockAdapters {
    email_sender: MockEmailSender,
    notifier: MockNotifier,
}

impl MockAdapters {
    pub fn email_sender_mut(&mut self) -> &mut MockEmailSender {
        &mut self.email_sender
    }

    pub fn notifier_mut(&mut self) -> &mut MockNotifier {
        &mut self.notifier
    }
}

impl Adapters for MockAdapters {
    type EmailSenderImpl = MockEmailSender;
    type NotifierImpl = MockNotifier;

    fn email_sender(&self) -> &Self::EmailSenderImpl {
        &self.email_sender
    }

    fn notifier(&self) -> &Self::NotifierImpl {
        &self.notifier
    }
}
