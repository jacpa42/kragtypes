pub mod password;
pub mod permissions;

use crate::args::create::CreateUser;
use crate::args::query::QueryUser;
use crate::permissions::Permissions;
use crate::table::BindValues;
use crate::{email::EmailAddr, table::Queryable};
use axum_login::AuthUser;
use backend_proc_macro::BindValues;
use password::PasswordHash;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

pub type PhoneNumber = i64;
pub type UserId = i32;
pub type PassId = i64;

#[derive(Clone, Deserialize, sqlx::FromRow, PartialEq, BindValues)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub email: EmailAddr,
    pub number: Option<PhoneNumber>,
    pub password: PasswordHash,
    pub permissions: Permissions,
}

impl Queryable for User {
    type CreateArgs = CreateUser;
    type QueryArgs = QueryUser;
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            username: String::new(),
            email: "default@email.com".parse().unwrap(),
            number: None,
            password: PasswordHash::from_raw(""),
            permissions: Permissions::NONE,
        }
    }
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("User", 5)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("username", &self.username)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("number", &self.number)?;
        state.serialize_field("permissions", &self.permissions)?;
        state.serialize_field("password", &"[REDACTED]")?;
        state.end()
    }
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("email", &self.email)
            .field("phone_number", &self.number)
            .field("permissions", &self.permissions)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = UserId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_ref()
        // We use the password hash as the auth
        // hash--what this means
        // is when the user changes their password the
        // auth session becomes invalid.
    }
}