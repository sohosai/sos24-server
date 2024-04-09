use getset::Getters;

use crate::entity::actor::Actor;
use crate::entity::permission::Permissions;

#[derive(Debug, Clone, PartialEq, Eq, Getters, Default)]
pub struct ProjectApplicationPeriod {
    #[getset(get = "pub")]
    start_at: chrono::DateTime<chrono::Utc>,
    #[getset(get = "pub")]
    end_at: chrono::DateTime<chrono::Utc>,
}

impl ProjectApplicationPeriod {
    pub fn new(start_at: String, end_at: String) -> Self {
        Self {
            start_at: start_at.parse().unwrap(),
            end_at: end_at.parse().unwrap(),
        }
    }

    pub fn can_create_project(&self, actor: &Actor, datetime: &chrono::DateTime<chrono::Utc>) -> bool {
        if actor.has_permission(Permissions::CREATE_PROJECT_ANYTIME) {
            return true;
        }

        &self.start_at <= datetime && datetime <= &self.end_at
    }
}
