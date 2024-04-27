use self::email::MockEmailSender;

pub mod email;

pub trait Adapters: Send + Sync + 'static {
    type EmailSenderImpl: email::EmailSender;

    fn email_sender(&self) -> &Self::EmailSenderImpl;
}

#[derive(Default)]
pub struct MockAdapters {
    email_sender: MockEmailSender,
}

impl MockAdapters {
    pub fn email_sender_mut(&mut self) -> &mut MockEmailSender {
        &mut self.email_sender
    }
}

impl Adapters for MockAdapters {
    type EmailSenderImpl = MockEmailSender;

    fn email_sender(&self) -> &Self::EmailSenderImpl {
        &self.email_sender
    }
}
