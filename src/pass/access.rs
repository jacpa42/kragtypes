use super::UserPass;

#[derive(PartialEq, Debug)]
pub enum AccessMethod {
    /// The user has a time pass which expires in the future.
    TimePass,
    /// The user has a session pass which has been used and no grace period.
    SessionPassSession,
    /// The user has an active grace period for their session pass.
    SessionPassGrace,
}

#[derive(PartialEq, Debug)]
pub enum AccessAttempt {
    Successful(AccessMethod),
    Failure,
}

impl AccessAttempt {
    pub fn is_success_and(&self, f: impl FnOnce(&AccessMethod) -> bool) -> bool {
        match self {
            AccessAttempt::Successful(access_method) => f(access_method),
            AccessAttempt::Failure => false,
        }
    }
}

pub trait Pass {
    /// Checks the key to see if it is valid and returns the status of the access attempt.
    fn use_key(&mut self) -> AccessAttempt;
}

impl Pass for UserPass {
    fn use_key(&mut self) -> AccessAttempt {
        match self.time_pass.use_key() {
            AccessAttempt::Successful(access_method) => AccessAttempt::Successful(access_method),
            AccessAttempt::Failure => self.session_pass.use_key(),
        }
    }
}

#[test]
fn pass() {
    use super::{SessionPass, TimePass};
    use chrono::{Days, Utc};

    let mut mem = UserPass {
        id: 0,
        user_id: 0,
        time_pass: TimePass {
            expiry: Utc::now().checked_add_days(Days::new(1)).unwrap(),
        },
        session_pass: SessionPass::default(),
    };

    mem.session_pass.sessions_left = 1;

    assert!(dbg!(mem.use_key()).is_success_and(|method| method == &AccessMethod::TimePass));
    assert!(dbg!(mem.session_pass.sessions_left) == 1);
    mem.time_pass = TimePass::default();

    dbg!(&mem);
    assert!(
        dbg!(mem.use_key()).is_success_and(|method| method == &AccessMethod::SessionPassSession)
    );
    assert!(dbg!(mem.session_pass.sessions_left) == 0);
    assert!(dbg!(mem.use_key()).is_success_and(|method| method == &AccessMethod::SessionPassGrace));
    assert!(dbg!(mem.session_pass.sessions_left) == 0);
    mem.session_pass = SessionPass::default();

    assert!(dbg!(mem.use_key()) == AccessAttempt::Failure);
}
