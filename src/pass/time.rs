use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, Sqlite, Type};
use std::str::FromStr;

use super::access::{AccessAttempt, AccessMethod, Pass};

#[repr(C)]
#[derive(Serialize, PartialEq, Deserialize, Debug, Clone, Copy)]
pub struct TimePass {
    pub expiry: DateTime<Utc>,
}

impl Type<Sqlite> for TimePass {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <[u8] as Type<Sqlite>>::type_info()
    }
}

impl<'q> Encode<'q, Sqlite> for TimePass {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        assert_eq!(12, std::mem::size_of::<Self>());
        let raw: &[u8; 12] = unsafe { std::mem::transmute(self) };
        Encode::<Sqlite>::encode(raw.as_slice(), buf)
    }
}

impl<'q> Decode<'q, Sqlite> for TimePass {
    fn decode(
        value: <Sqlite as sqlx::Database>::ValueRef<'q>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes: &[u8] = Decode::<Sqlite>::decode(value)?;
        assert_eq!(bytes.len(), std::mem::size_of::<Self>());
        let tp_pointer: *const TimePass = unsafe { std::mem::transmute(bytes.as_ptr()) };
        Ok(unsafe { *tp_pointer })
    }
}

impl FromStr for TimePass {
    type Err = chrono::ParseError;
    /// Parses a time into an expiry date for the pass. The format is %Y-%m-%d %H:%M:%S and must be
    /// in UTC format!!
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TimePass {
            expiry: DateTime::<Utc>::from_str(s)?,
        })
    }
}

impl Default for TimePass {
    /// A pass which expired in 1970 (`UNIX_EPOCH`)
    fn default() -> Self {
        Self {
            expiry: DateTime::<Utc>::UNIX_EPOCH,
        }
    }
}

impl Pass for TimePass {
    fn use_key(&mut self) -> AccessAttempt {
        if Utc::now() < self.expiry {
            AccessAttempt::Successful(AccessMethod::TimePass)
        } else {
            AccessAttempt::Failure
        }
    }
}
