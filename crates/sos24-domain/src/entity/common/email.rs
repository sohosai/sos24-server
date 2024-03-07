use thiserror::Error;

#[derive(Debug)]
pub struct Email(String);

impl Email {
    pub fn value(self) -> String {
        self.0
    }
}

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Invalid email format")]
    InvalidFormat,
    #[error("Invalid email domain")]
    InvalidDomain,
}

impl TryFrom<String> for Email {
    type Error = EmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !is_valid_email_format(&value) {
            return Err(EmailError::InvalidFormat);
        }
        if !value.ends_with("u.tsukuba.ac.jp") {
            return Err(EmailError::InvalidDomain);
        }
        Ok(Self(value))
    }
}

fn is_valid_email_format(email: &str) -> bool {
    // https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address
    let email_re = regex::Regex::new(r"^[a-zA-Z0-9.!#$%&'*+\/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
    email_re.is_match(email)
}

#[cfg(test)]
mod test {
    #[test]
    fn valid_email_format() {
        assert!(super::is_valid_email_format("s0000000@u.tsukuba.ac.jp"));
        assert!(super::is_valid_email_format("john.doe@example.jp"));
    }

    #[test]
    fn invalid_email_format() {
        assert!(!super::is_valid_email_format("s@@example.com"));
        assert!(!super::is_valid_email_format("s@u."));
        assert!(!super::is_valid_email_format("example.com"));
    }
}
