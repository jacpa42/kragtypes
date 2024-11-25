#![cfg(feature = "sqlite")]
use crate::table::Queryable;
use serde::{Deserialize, Serialize};

/// The type expected when updating a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update<A>
where
    A: Queryable,
{
    pub match_params: A::QueryArgs,
    pub new_params: A::QueryArgs,
}
