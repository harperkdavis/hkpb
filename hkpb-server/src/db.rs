use std::path::Path;

use sqlx::{sqlite::SqliteConnectOptions, Executor, SqlitePool};

pub type Db = SqlitePool;

pub async fn db_connect(filename: impl AsRef<Path>) -> color_eyre::Result<SqlitePool> {
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    let db = SqlitePool::connect_with(options).await?;

    db.execute(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT UNIQUE PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            initialized BOOLEAN NOT NULL
        )
        "#,
    ).await?;

    Ok(db)
}