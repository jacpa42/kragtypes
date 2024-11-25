use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::access::{AccessAttempt, AccessMethod, Pass};

#[repr(C)]
#[derive(Serialize, PartialEq, Deserialize, Debug, Clone, Copy)]
pub struct TimePass {
    pub expiry: DateTime<Utc>,
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
