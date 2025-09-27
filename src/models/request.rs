use anyhow::Result;
use serde::Deserialize;

use super::db::User;

#[cfg(feature = "migrate-md5")]
pub const MD5_LEN: usize = 32;

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct Auth {
    pub email: String,
    pub password: String,
    #[cfg(feature = "migrate-md5")]
    #[serde(skip_deserializing)]
    pub password_md5: String,
}

impl Auth {
    #[cfg(test)]
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_owned(),
            password: password.to_owned(),
            #[cfg(feature = "migrate-md5")]
            password_md5: "".to_string(),
        }
    }

    /// Validate fields and hash password
    pub fn parse(&self) -> Result<Self, &'static str> {
        if !matches!(self.password.len(), 2..=24) {
            return Err("Password should be from 2 to 24 characters long");
        }
        if !matches!(self.email.len(), 5..=320) || !self.email.contains('@') {
            return Err("Invalid email address");
        }

        Ok(Self {
            email: self.email.clone(),
            password: blake3::hash(self.password.as_bytes()).to_string(),
            #[cfg(feature = "migrate-md5")]
            password_md5: to_md5(&self.password),
        })
    }

    pub fn check_password(&self, user: &User) -> Result<(), ()> {
        if self.password == user.password_hash {
            return Ok(());
        }
        // todo: also migrate new passwords for "original"
        #[cfg(feature = "migrate-md5")]
        if user.password_hash.len() == MD5_LEN && self.password_md5 == user.password_hash {
            return Ok(());
        }

        Err(())
    }
}

#[cfg(feature = "migrate-md5")]
pub fn to_md5(input: &str) -> String {
    use md5::Digest;

    let mut hasher = md5::Md5::new();
    hasher.update(input);

    // in database md5 hashes stored not in hex form
    String::from_utf8_lossy(hasher.finalize().as_slice()).to_string()
}

#[cfg(all(test, feature = "migrate-md5"))]
#[test]
fn test_to_md5() {
    let res = to_md5("test");
    assert_eq!(res.len(), MD5_LEN);
    assert_eq!(res, "\t�k�F!�s��N�&'��");
}
