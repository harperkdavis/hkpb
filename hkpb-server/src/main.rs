use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::{services::ServeDir, trace::{DefaultMakeSpan, TraceLayer}};
use tracing::info;

mod db;
mod session;

fn tracing_init() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hkpb_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[derive(Debug, Clone)]
pub struct InnerAppState {
    db: db::Db,
}

pub type AppState = Arc<InnerAppState>;

#[tokio::main]
async fn main() {
    use std::net::{IpAddr, Ipv6Addr, SocketAddr};

    tracing_init();

    info!("starting hkpb-server...");

    let db = db::db_connect("hkpb.db")
        .await
        .expect("failed to connect to database");

    let session_layer = session::session_layer(db.clone()).await;

    let app_state = Arc::new(InnerAppState { db });

    let serve_dir = ServeDir::new("../hkpb-client/dist/")
        .append_index_html_on_directories(true);

    let router = Router::new()
        .nest_service("/", serve_dir)
        .route("/hello", get(|| async { "Hello, World!" }))
        .with_state(app_state)
        .layer(session_layer)
        .layer(TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::default().include_headers(true)));
    
    let addr = SocketAddr::new(IpAddr::from(Ipv6Addr::LOCALHOST), 8080);

    info!("listening on {}", addr);

    axum_server::bind(addr)
        .serve(router.into_make_service())
        .await
        .expect("failed to start server");
}
