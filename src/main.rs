use axum::{Router, routing::get};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;

mod handler;
mod types;
mod utils;

use handler::spotify::SpotifyTokenHandler;
use utils::logger::logs;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("Invalid address");
    let token_handler = Arc::new(SpotifyTokenHandler::new());
    // Fetch token awal saat start
    token_handler.init(|| handler::token::extract_token()).await;
    let app = Router::new().route(
        "/spotifytoken",
        get(handler::token::handle_token).with_state(token_handler.clone()),
    );
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind");
    logs("info", &[&format!("Server started on {}", addr)]);
    tokio::select! {
        _ = axum::serve(listener, app.into_make_service_with_connect_info::<std::net::SocketAddr>()) => {},
        _ = signal::ctrl_c() => {
            logs("info", &[&"Shutdown signal received"]);
        }
    }
}
