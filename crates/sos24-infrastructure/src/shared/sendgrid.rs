use std::ops::Deref;

use sendgrid::v3::Sender;

pub struct SendGrid(Sender);

impl SendGrid {
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        Self(Sender::new(api_key.into(), None))
    }
}

impl Deref for SendGrid {
    type Target = Sender;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
