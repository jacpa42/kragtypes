#![cfg(feature = "sqlite")]

pub mod user;
pub mod user_pass;

use serde::Serialize;
use smol_str::SmolStr;
use sqlx::{
    prelude::FromRow,
    query::Query,
    sqlite::{SqliteArguments, SqliteQueryResult, SqliteRow},
    Pool, Sqlite,
};

/// Any type which implements this trait can be stored in the database. It provides the table name
/// as well as the name of the columns and their types.
pub trait Table: Sized {
    /// Returns the table name of type in the database
    fn table_name() -> SmolStr;
    // Returns the names of the columns in the database
    fn column_names() -> Vec<SmolStr>;
    /// Returns the initialization statement for the type to create its table in the database.
    fn init(
        pool: &Pool<Sqlite>,
    ) -> impl std::future::Future<Output = Result<SqliteQueryResult, sqlx::Error>> + Send;
}

pub trait BindValues {
    /// Returns the values of the bound columns as well as their names
    fn bind_values<'q>(
        &'q self,
        query: Query<'q, Sqlite, SqliteArguments<'q>>,
    ) -> Query<'q, Sqlite, SqliteArguments<'q>>;
    fn bound_values(&self) -> Vec<SmolStr>;
}

pub trait Queryable:
    std::fmt::Debug + Serialize + BindValues + Table + for<'r> FromRow<'r, SqliteRow> + Send + Unpin
where
    Self::CreateArgs: BindValues + std::fmt::Debug,
    Self::QueryArgs: BindValues + std::fmt::Debug,
{
    type CreateArgs;
    type QueryArgs;
}
