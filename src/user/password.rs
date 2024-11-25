use password_auth::generate_hash;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Clone, Serialize, PartialEq)]
#[cfg_attr(feature = "sqlite", derive(sqlx::FromRow))]
pub struct PasswordHash {
    pub(crate) data: String,
}

impl PasswordHash {
    /// Takes a plain password and hashes it
    pub fn from_raw(password: impl AsRef<[u8]>) -> Self {
        Self {
            data: generate_hash(password),
        }
    }

    /// # Safety
    /// The developer must pass valid utf8 to this struct. This will store the plain password when called,
    /// hence it should almost never be used in production. I have included it for testing
    pub unsafe fn with_no_hash(password: impl AsRef<[u8]>) -> Self {
        Self {
            data: String::from_utf8_unchecked(password.as_ref().to_vec()),
        }
    }
}

impl<'de> Deserialize<'de> for PasswordHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let password_raw = String::deserialize(deserializer)?;
        Ok(Self {
            data: generate_hash(password_raw),
        })
    }
}

impl AsRef<str> for PasswordHash {
    fn as_ref(&self) -> &str {
        self.data.as_ref()
    }
}

impl AsRef<[u8]> for PasswordHash {
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl FromStr for PasswordHash {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            data: generate_hash(s),
        })
    }
}
