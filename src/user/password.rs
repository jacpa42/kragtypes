use std::{convert::Infallible, str::FromStr};

use password_auth::generate_hash;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, FromRow, Sqlite, Type};

#[derive(Debug, Clone, Serialize, FromRow, PartialEq)]
pub struct PasswordHash {
    data: String,
}

impl PasswordHash {
    /// Takes a plain password and hashes it
    pub fn from_raw(password: impl AsRef<[u8]>) -> Self {
        Self {
            data: generate_hash(password),
        }
    }
}

impl<'r> Encode<'r, Sqlite> for PasswordHash {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'r>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <String as Encode<Sqlite>>::encode_by_ref(&self.data, buf)
    }
}

impl<'r> Decode<'r, Sqlite> for PasswordHash {
    fn decode(
        value: <Sqlite as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let data = <String as Decode<Sqlite>>::decode(value)?;
        Ok(Self { data })
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

impl Type<Sqlite> for PasswordHash {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <str as Type<Sqlite>>::type_info()
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
