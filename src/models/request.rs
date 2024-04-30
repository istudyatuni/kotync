use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct Auth {
    pub email: String,
    pub password: String,
}

impl Auth {
    /// Validate fields and hash password
    pub fn parse(&self) -> Result<Self, &'static str> {
        let mut s = self.clone();
        if !matches!(s.password.len(), 2..=24) {
            return Err("Password should be from 2 to 24 characters long");
        }
        if !matches!(s.email.len(), 5..=120) || !s.email.contains('@') {
            return Err("Invalid email address");
        }

        s.password = blake3::hash(s.password.as_bytes()).to_string();

        Ok(s)
    }
}
