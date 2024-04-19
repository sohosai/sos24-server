use sos24_use_case::context::ContextProvider;

#[derive(Clone)]
pub struct Context {
    user_id: String,
    requested_at: chrono::DateTime<chrono::Utc>,
}

impl Context {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            requested_at: chrono::Utc::now(),
        }
    }
}

impl ContextProvider for Context {
    fn user_id(&self) -> String {
        self.user_id.clone()
    }

    fn requested_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.requested_at
    }
}
