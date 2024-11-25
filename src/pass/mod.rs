pub mod access;
pub mod session;
pub mod sqlx_impl;
pub mod time;

use serde::{Deserialize, Serialize};
use session::SessionPass;
use time::TimePass;

use crate::user::{PassId, UserId};

#[cfg(feature = "sqlite")]
use {
    crate::table::{BindValues, Queryable},
    backend_proc_macro::BindValues,
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "sqlite", derive(sqlx::FromRow, BindValues))]
pub struct UserPass {
    pub id: PassId,
    pub user_id: UserId,
    pub time_pass: TimePass,
    pub session_pass: SessionPass,
}

#[cfg(feature = "sqlite")]
impl Queryable for UserPass {
    type CreateArgs = crate::args::create::CreateUserPass;
    type QueryArgs = crate::args::query::QueryUserPass;
}
