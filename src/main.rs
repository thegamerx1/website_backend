use std::net::SocketAddr;

use axum::Server;
use axum::{http::Method, routing::get};
use chrono::Utc;
use serde::Serialize;
use serde_json::Value;
use socketioxide::{Namespace, SocketIoLayer};
use tower_http::cors;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[derive(Serialize)]
struct ServerTimeStamp {
    time_server: i64,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("APP_PORT")
        .unwrap_or("8080".to_owned())
        .parse::<u16>()?;
    let host = std::env::var("APP_HOST").unwrap_or("127.0.0.1".to_owned());
    let allowed_origin =
        std::env::var("ALLOWED_ORIGIN").unwrap_or("http://localhost:5173".to_owned());
    let socket = SocketAddr::new(host.parse()?, port);

    let allow_origin = cors::AllowOrigin::list([
        allowed_origin.parse().expect("Valid allowed_origin"),
        "https://192.168.1.69:5173".parse().expect("Vlaid "),
        "http://192.168.1.69:5173".parse().expect("Vlaid "),
    ]);

    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let ns = Namespace::builder()
        .add("/time", |socket| async move {
            info!("connection /time: {:?} {:?}", socket.ns(), socket.sid);

            socket.on("lag", |_socket, data: i64, _, sender| async move {
                let received: i64 = Utc::now().timestamp_millis();
                sender.send(received).ok();
            });
        })
        .add("/song", |socket| async move {
            info!("Socket.IO connected on: {:?} {:?}", socket.ns(), socket.sid);
            let data: Value = socket.handshake.data().unwrap();
            socket.emit("auth", data).ok();
        })
        .build();

    let cors = cors::CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(vec![Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(allow_origin);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(SocketIoLayer::new(ns))
        .layer(cors);

    info!("Starting server");

    Server::bind(&socket).serve(app.into_make_service()).await?;

    Ok(())
}
