use super::Table;
use crate::pass::UserPass;
use smol_str::SmolStr;

impl Table for UserPass {
    fn table_name() -> smol_str::SmolStr {
        SmolStr::from("userpass")
    }

    fn column_names() -> Vec<smol_str::SmolStr> {
        vec![
            SmolStr::from("id"),
            SmolStr::from("user_id"),
            SmolStr::from("time_pass"),
            SmolStr::from("session_pass"),
        ]
    }

    async fn init(
        pool: &sqlx::Pool<sqlx::Sqlite>,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS userpass (
                id BIGINT PRIMARY KEY,
                user_id INTEGER NOT NULL,
                time_pass BLOB NOT NULL,
                session_pass BLOB NOT NULL
            )",
        )
        .execute(pool)
        .await
    }
}
