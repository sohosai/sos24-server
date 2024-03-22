use thiserror::Error;

use crate::impl_value_object;

impl_value_object!(DateTime(chrono::DateTime<chrono::Utc>));

#[derive(Debug, Error)]
pub enum DateTimeError {
    #[error("Invalid datetime format")]
    InvalidFormat,
}

impl TryFrom<String> for DateTime {
    type Error = DateTimeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let dt = chrono::DateTime::parse_from_rfc3339(&value)
            .map_err(|_| DateTimeError::InvalidFormat)?;
        Ok(Self(dt.with_timezone(&chrono::Utc)))
    }
}
