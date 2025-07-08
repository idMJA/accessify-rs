use crate::types::spotify::SpotifyToken;
use chrono;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::{Duration, Instant, sleep_until};

pub struct SpotifyTokenHandler {
    semaphore: Arc<Semaphore>,
    cached_token: Arc<Mutex<Option<SpotifyToken>>>,
}

impl SpotifyTokenHandler {
    pub fn new() -> Self {
        Self {
            semaphore: Arc::new(Semaphore::const_new(1)),
            cached_token: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn get_access_token<F, Fut>(&self, extract_token: F) -> Result<String, String>
    where
        F: Fn() -> Fut + Send + Sync,
        Fut: std::future::Future<Output = Result<String, String>>,
    {
        // Cek cache
        {
            let token_guard = self.cached_token.lock().await;
            if let Some(token) = &*token_guard {
                if token.expires_at > Instant::now() {
                    return Ok(token.value.clone());
                }
            }
        }
        // Acquire semaphore
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| "Semaphore error")?;
        // Cek lagi setelah dapat permit (race condition)
        {
            let token_guard = self.cached_token.lock().await;
            if let Some(token) = &*token_guard {
                if token.expires_at > Instant::now() {
                    return Ok(token.value.clone());
                }
            }
        }
        // Fetch token baru
        let token_json = extract_token().await?;
        let expires_in = match serde_json::from_str::<serde_json::Value>(&token_json) {
            Ok(val) => val
                .get("accessTokenExpirationTimestampMs")
                .and_then(|v| v.as_i64())
                .map(|ts| ts - chrono::Utc::now().timestamp_millis())
                .unwrap_or(10000),
            Err(_) => 10000,
        };
        let expires_at = Instant::now() + Duration::from_millis(expires_in.max(1000) as u64);
        {
            let mut token_guard = self.cached_token.lock().await;
            *token_guard = Some(SpotifyToken {
                value: token_json.clone(),
                expires_at,
            });
        }
        // Schedule refresh otomatis
        let cached_token = self.cached_token.clone();
        tokio::spawn(async move {
            sleep_until(expires_at + Duration::from_millis(100)).await;
            let mut token_guard = cached_token.lock().await;
            *token_guard = None;
            crate::utils::logger::logs("info", &[&"Spotify token auto-refreshed (timeout)"]);
        });
        Ok(token_json)
    }

    pub async fn init<F, Fut>(&self, extract_token: F)
    where
        F: Fn() -> Fut + Send + Sync,
        Fut: std::future::Future<Output = Result<String, String>>,
    {
        match self.get_access_token(extract_token).await {
            Ok(_) => crate::utils::logger::logs("info", &[&"Initial Spotify token fetched"]),
            Err(e) => crate::utils::logger::logs(
                "error",
                &[&format!("Failed to fetch initial Spotify token: {}", e)],
            ),
        }
    }
}
