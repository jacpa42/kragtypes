use crate::{
    email::EmailAddr,
    pass::{session::SessionPass, time::TimePass},
    user::{PassId, PhoneNumber, UserId},
};
use serde::{Deserialize, Serialize};
#[cfg(feature = "sqlite")]
use {crate::table::BindValues, backend_proc_macro::BindValues};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlite", derive(BindValues))]
pub struct QueryUser {
    pub id: Option<UserId>,
    pub username: Option<String>,
    pub email: Option<EmailAddr>,
    pub number: Option<PhoneNumber>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlite", derive(BindValues))]
pub struct QueryUserPass {
    pub id: Option<PassId>,
    pub user_id: Option<UserId>,
    pub time_pass: Option<TimePass>,
    pub session_pass: Option<SessionPass>,
}
