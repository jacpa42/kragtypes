use crate::table::BindValues;
use crate::{
    email::EmailAddr,
    pass::{session::SessionPass, time::TimePass},
    user::{password::PasswordHash, permissions::Permissions, PassId, PhoneNumber, UserId},
};
use backend_proc_macro::BindValues;
use serde::{Deserialize, Serialize};

/// The type expected when creating a user.
#[derive(Debug, Clone, Serialize, Deserialize, BindValues)]
pub struct CreateUser {
    pub id: Option<UserId>,
    pub username: String,
    pub email: EmailAddr,
    pub number: Option<PhoneNumber>,
    pub password: PasswordHash,
    pub permissions: Permissions,
}

/// The type expected when creating a user.
#[derive(Debug, Clone, Serialize, Deserialize, BindValues)]
pub struct CreateUserPass {
    pub id: Option<PassId>,
    pub user_id: UserId,
    pub time_pass: TimePass,
    pub session_pass: SessionPass,
}
