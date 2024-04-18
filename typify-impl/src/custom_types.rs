use std::str::FromStr;

use regress::Regex;

#[derive(Debug)]
pub struct Email(pub String);

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for Email {
    type Err = super::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new("^\\S+@\\S+\\.\\S+$").unwrap();

        if s.len() < 6 {
            return Err(crate::Error::InvalidSchema {
                type_name: Some("email".to_string()),
                reason: "An email needs to be at least 6 characters".to_string(),
            });
        } else if 128 < s.len() {
            return Err(crate::Error::InvalidSchema {
                type_name: Some("email".to_string()),
                reason: "An email needs to be less than 128 characters".to_string(),
            });
        } else if regex.find(s).is_none() {
            return Err(crate::Error::InvalidSchema {
                type_name: Some("email".to_string()),
                reason: "This is not a correctly formatted email".to_string(),
            });
        }

        Ok(Email(s.to_string()))
    }
}

impl From<String> for Email {
    fn from(value: String) -> Self {
        Email(value)
    }
}

impl From<&str> for Email {
    fn from(value: &str) -> Self {
        Email(value.to_string())
    }
}
