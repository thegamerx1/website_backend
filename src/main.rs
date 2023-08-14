use axum::http::HeaderValue;
use dotenv::dotenv;
use std::{env, net::SocketAddr};
use tower_http::cors;
use webserver::webserver;

mod webserver;

fn get_env(name: &str, default: &str) -> String {
    env::var(name).unwrap_or(default.to_string())
}

fn get_env_fail(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("{} environment variable missing.", name))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let port = get_env("APP_PORT", "8080").parse::<u16>()?;
    let host = get_env("APP_HOST", "127.0.0.1");
    let allowed_origin = get_env("ALLOWED_ORIGIN", "http://localhost:5173")
        .split(" ")
        .map(|str| str.parse().expect("Valid ALLOWED_ORIGIN"))
        .collect::<Vec<HeaderValue>>();

    // let webhook_url: String = get_env_fail("WEBHOOK_URL");

    webserver(
        SocketAddr::new(host.parse()?, port),
        cors::AllowOrigin::list(allowed_origin),
    )
    .await?;
    Ok(())
}
