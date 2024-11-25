use crate::args::{create::CreateUserPass, query::QueryUserPass};
use crate::table::{BindValues, Queryable};
use backend_proc_macro::BindValues;
use serde::{Deserialize, Serialize};
use session::SessionPass;
use time::TimePass;

use crate::user::{PassId, UserId};

pub mod access;
pub mod session;
pub mod time;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone, Copy, BindValues, PartialEq)]
pub struct UserPass {
    pub id: PassId,
    pub user_id: UserId,
    pub time_pass: TimePass,
    pub session_pass: SessionPass,
}

impl Queryable for UserPass {
    type CreateArgs = CreateUserPass;
    type QueryArgs = QueryUserPass;
}
