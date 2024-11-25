use super::access::{AccessAttempt, AccessMethod, Pass};
use chrono::{DateTime, Duration, FixedOffset, Timelike, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, Decode, Encode, Sqlite};
use std::str::FromStr;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct SessionPass {
    pub last_time_used: DateTime<Utc>,
    pub sessions_left: u32,
}

impl Default for SessionPass {
    /// A pass with 0 sessions and the `last_time_used = UNIX_EPOCH`.
    fn default() -> Self {
        Self {
            sessions_left: 0,
            last_time_used: DateTime::<Utc>::UNIX_EPOCH,
        }
    }
}

impl Type<Sqlite> for SessionPass {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <[u8] as Type<Sqlite>>::type_info()
    }
}

impl<'q> Encode<'q, Sqlite> for SessionPass {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        assert_eq!(16, std::mem::size_of::<Self>());
        let raw: &[u8; 16] = unsafe { std::mem::transmute(self) };
        Encode::<Sqlite>::encode(raw.as_slice(), buf)
    }
}

impl<'q> Decode<'q, Sqlite> for SessionPass {
    fn decode(
        value: <Sqlite as sqlx::Database>::ValueRef<'q>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes: &[u8] = Decode::<Sqlite>::decode(value)?;
        assert_eq!(bytes.len(), std::mem::size_of::<Self>());
        let tp_pointer: *const SessionPass = unsafe { std::mem::transmute(bytes.as_ptr()) };
        Ok(unsafe { *tp_pointer })
    }
}

impl FromStr for SessionPass {
    type Err = std::num::ParseIntError;
    /// Parses the number of sessions from a string slice.
    ///
    /// # Example
    /// ```
    /// use kragdb::pass::session::SessionPass;
    /// use std::str::FromStr;
    ///
    /// let session_pass = SessionPass {
    ///     sessions_left: 4,
    ///     last_time_used: chrono::DateTime::<chrono::Utc>::UNIX_EPOCH,
    /// };
    /// let session_pass_from_str = SessionPass::from_str("4");
    ///
    /// assert!(session_pass_from_str.is_ok_and(|pass| pass == session_pass));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            sessions_left: s.parse()?,
            last_time_used: chrono::DateTime::<Utc>::UNIX_EPOCH,
        })
    }
}

impl Pass for SessionPass {
    /// If a user consumes a session, the time at which entering will consume another session is
    /// outlined in the following table:
    /// ```md
    /// | Sign in time  | Free entry until   |
    /// |---------------|--------------------|
    /// | 00:00 - 19:59 | 23:59 __same__ day |
    /// | 20:00 - 23:59 | 05:00 __next__ day |
    ///
    /// ```
    fn use_key(&mut self) -> AccessAttempt {
        let one_day = 86400;
        let hour = 3600;
        let local_time = self
            .last_time_used
            .with_timezone(&FixedOffset::east_opt(2 * hour).unwrap());
        let expiry = {
            let next_midnight = local_time
                + Duration::seconds(one_day - local_time.num_seconds_from_midnight() as i64);

            match local_time.hour() {
                0..20 => next_midnight,
                _ => next_midnight + Duration::hours(5),
            }
        };

        let now = Utc::now().with_timezone(&FixedOffset::east_opt(2 * hour).unwrap());

        if now < expiry {
            AccessAttempt::Successful(AccessMethod::SessionPassGrace)
        } else if self.sessions_left > 0 {
            self.sessions_left -= 1;
            self.last_time_used = now.to_utc();
            AccessAttempt::Successful(AccessMethod::SessionPassSession)
        } else {
            AccessAttempt::Failure
        }
    }
}

#[test]
fn session_pass() {
    use chrono::{Duration, Local};
    let mut pass = SessionPass {
        sessions_left: 3,
        ..Default::default()
    };

    assert!(pass
        .use_key()
        .is_success_and(|method| method == &AccessMethod::SessionPassSession));
    assert!(pass.sessions_left == 2);

    assert!(pass
        .use_key()
        .is_success_and(|method| method == &AccessMethod::SessionPassGrace));
    assert!(pass.sessions_left == 2);

    assert!(Local::now().signed_duration_since(pass.last_time_used) < Duration::seconds(1));

    let mut pass = SessionPass::default();
    let inital_last_use = pass.last_time_used;

    assert!(pass.use_key() == AccessAttempt::Failure);
    assert!(pass.sessions_left == 0);
    assert!(pass.use_key() == AccessAttempt::Failure);
    assert!(pass.sessions_left == 0);

    assert!(pass.last_time_used == inital_last_use);
}
