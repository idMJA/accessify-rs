use axum::{Router, routing::get};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tracing_subscriber::EnvFilter;
mod handler;
mod types;
mod utils;
use axum_extra::extract::TypedHeader;
use handler::spotify::SpotifyTokenHandler;
use handler::token::handle_token;
use headers::UserAgent;
use utils::logger::logs;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("chromiumoxide=off,accessify_rs=info"))
        .init();
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("Invalid address");
    let token_handler = Arc::new(SpotifyTokenHandler::new());
    // Fetch token awal saat start
    token_handler.init(|| handler::token::extract_token()).await;
    let app = Router::new().route(
        "/spotifytoken",
        get(
            |user_agent: TypedHeader<UserAgent>, axum::extract::State(token_handler)| async move {
                handle_token(user_agent, token_handler).await
            },
        )
        .with_state(token_handler.clone()),
    );
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind");
    logs("info", &[&format!("Server started on {}", addr)]);
    tokio::select! {
        _ = axum::serve(listener, app) => {},
        _ = signal::ctrl_c() => {
            logs("info", &[&"Shutdown signal received"]);
        }
    }
}
