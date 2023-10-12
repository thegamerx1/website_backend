// use std::sync::Arc;
// use axum::extract::State;
// use axum::routing::post;
// use serde::Deserialize;

// use serde_json::Value;
// use socketioxide::{Namespace, SocketIoLayer};
// use webhook::client::WebhookClient;

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

    // let ns = Namespace::builder()
    //     .add("/time", |socket| async move {
    //         info!("connection /time: {:?}", socket.ns());

    //         socket.on("lag", |_socket, _: Value, _, sender| async move {
    //             let received: i64 = Utc::now().timestamp_millis();
    //             sender.send(received).ok();
    //         });
    //     })
    //     .build();

    let cors = CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_methods(vec![Method::GET, Method::POST]);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/lag", get(lag))
        // .route("/contact", post(contact_submit))
        // .with_state(state)
        // .layer(SocketIoLayer::new(ns))
        .layer(cors);

    info!("Starting server");

    Server::bind(&socket).serve(app.into_make_service()).await?;

    Ok(())
}

// #[derive(Deserialize)]
// struct ContactForm {
//     name: String,
//     email: String,
//     body: String,
// }

// #[axum::debug_handler]
// async fn contact_submit(
//     state: State<AppState>,
//     Json(payload): Json<ContactForm>,
// ) -> (StatusCode, String) {
//     let client = &state.webhook;
//     match client
//         .send(|message| {
//             message.username("Contact form").embed(|embed| {
//                 embed
//                     .title("Contact")
//                     .author(&payload.name, None, None)
//                     .field("Email", &payload.email, false)
//                     .field("Text", &payload.body, false)
//             })
//         })
//         .await
//     {
//         Err(err) => {
//             let err = err.to_string();
//             eprintln!("{err}");
//             (StatusCode::INTERNAL_SERVER_ERROR, err)
//         }
//         Ok(_) => (StatusCode::OK, "OK".to_string()),
//     }
// }

async fn lag() -> String {
    let received: i64 = Utc::now().timestamp_millis();
    received.to_string()
}
