use smol_str::SmolStr;
use sqlx::{sqlite::SqliteQueryResult, Pool, Sqlite};

use super::Table;
use crate::user::User;

impl Table for User {
    fn table_name() -> SmolStr {
        "user".into()
    }

    fn column_names() -> Vec<SmolStr> {
        vec![
            "id".into(),
            "username".into(),
            "number".into(),
            "email".into(),
            "password".into(),
        ]
    }

    async fn init(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS user (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                number INTEGER UNIQUE,
                email TEXT NOT NULL UNIQUE,
                permissions INTEGER NOT NULL,
                password TEXT NOT NULL
            )",
        )
        .execute(pool)
        .await
    }
}
