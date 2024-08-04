use sqlx::SqlitePool;
use tower_sessions::{cookie::time::Duration, session_store::ExpiredDeletion, SessionManagerLayer};
use tower_sessions_sqlx_store::SqliteStore;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SessionData {
    pub user_id: String,
}

pub async fn session_layer(db: SqlitePool) -> SessionManagerLayer<SqliteStore> {
    let session_store = SqliteStore::new(db);
    session_store.migrate().await.unwrap();

    tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60 * 60))
    );

    SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(tower_sessions::Expiry::OnInactivity(Duration::days(7)))
}