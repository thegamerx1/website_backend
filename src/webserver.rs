use axum::http::HeaderValue;
use axum::Server;
use axum::{http::Method, routing::get};
use chrono::Utc;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

// #[derive(Clone)]
// struct AppState {
//     webhook: Arc<WebhookClient>,
// }

pub async fn webserver(
    socket: SocketAddr,
    allow_origin: Vec<HeaderValue>,
    // webhook_url: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // let state = AppState {
    //     webhook: Arc::new(WebhookClient::new(&webhook_url)),
    // };

    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let cors = CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_methods(vec![Method::GET, Method::POST]);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/lag", get(lag))
        .layer(cors);

    info!("Starting server");

    Server::bind(&socket).serve(app.into_make_service()).await?;

    Ok(())
}

async fn lag() -> String {
    let received: i64 = Utc::now().timestamp_millis();
    received.to_string()
}
