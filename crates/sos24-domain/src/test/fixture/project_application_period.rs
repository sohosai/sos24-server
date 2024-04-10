use crate::entity::project_application_period::ProjectApplicationPeriod;

pub fn applicable_period() -> ProjectApplicationPeriod {
    ProjectApplicationPeriod::new(
        chrono::Utc::now()
            .checked_sub_days(chrono::Days::new(1))
            .unwrap()
            .to_rfc3339(),
        chrono::Utc::now()
            .checked_add_days(chrono::Days::new(1))
            .unwrap()
            .to_rfc3339(),
    )
}

pub fn not_applicable_period() -> ProjectApplicationPeriod {
    ProjectApplicationPeriod::new(
        chrono::Utc::now()
            .checked_sub_days(chrono::Days::new(2))
            .unwrap()
            .to_rfc3339(),
        chrono::Utc::now()
            .checked_sub_days(chrono::Days::new(1))
            .unwrap()
            .to_rfc3339(),
    )
}
