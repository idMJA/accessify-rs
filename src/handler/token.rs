use crate::handler::spotify::SpotifyTokenHandler;
use crate::utils::logger::logs;
use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::TypedHeader;
use headers::UserAgent;
use std::sync::Arc;

pub async fn handle_token(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    token_handler: Arc<SpotifyTokenHandler>,
) -> impl IntoResponse {
    let start = std::time::Instant::now();
    let ip = "unknown"; // i gave up
    let ua = user_agent.as_str();
    let result = token_handler
        .get_access_token(|| crate::handler::token::extract_token())
        .await;
    let elapsed = start.elapsed().as_millis();
    logs(
        "info",
        &[&format!(
            "Handled Spotify Token request from IP: {}, UA: {} in {}ms",
            ip, ua, elapsed
        )],
    );
    match result {
        Ok(body) => ([("Content-Type", "application/json")], body).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to extract token").into_response(),
    }
}

pub async fn extract_token() -> Result<String, String> {
    use base64::prelude::*;
    use chromiumoxide::Browser;
    use chromiumoxide::cdp::browser_protocol::network::{
        EnableParams, EventResponseReceived, GetResponseBodyParams,
    };
    use futures::StreamExt;
    use serde_json::Value;
    use std::time::Duration;
    use tokio::time::{sleep, timeout};

    // Launch browser in headless mode with minimal args
    let (mut browser, mut handler) = Browser::launch(
        chromiumoxide::BrowserConfig::builder()
            .args(vec![
                "--headless=new",
                "--no-sandbox",
                "--disable-web-security",
                "--disable-dev-shm-usage",
                "--mute-audio",
            ])
            .build()
            .map_err(|e| format!("Browser config error: {e}"))?,
    )
    .await
    .map_err(|e| format!("Browser launch error: {e}"))?;

    // Handler task to process browser events silently
    let handler_task = tokio::spawn(async move { while let Some(_) = handler.next().await {} });

    // Create page
    let page = browser
        .new_page("about:blank")
        .await
        .map_err(|e| format!("Page creation error: {e}"))?;

    // Enable network events
    page.execute(EnableParams::default())
        .await
        .map_err(|e| format!("Network enable error: {e}"))?;

    // Get network event listener
    let mut network_events = page
        .event_listener::<EventResponseReceived>()
        .await
        .map_err(|e| format!("Event listener error: {e}"))?;

    // Navigate to Spotify
    page.goto("https://open.spotify.com/")
        .await
        .map_err(|e| format!("Navigation error: {e}"))?;

    // Wait for the token API response with timeout
    let token_result = timeout(Duration::from_secs(20), async {
        while let Some(event) = network_events.next().await {
            if event.response.url.contains("/api/token") {
                let request_id = event.request_id.clone();
                // Small delay to ensure response is complete
                sleep(Duration::from_millis(200)).await;

                if let Ok(response_body) =
                    page.execute(GetResponseBodyParams::new(request_id)).await
                {
                    let body_text = if response_body.base64_encoded {
                        match BASE64_STANDARD.decode(&response_body.body) {
                            Ok(decoded) => match String::from_utf8(decoded) {
                                Ok(text) => text,
                                Err(_) => continue,
                            },
                            Err(_) => continue,
                        }
                    } else {
                        response_body.body.clone()
                    };

                    // Parse and remove _notes field if present
                    return match serde_json::from_str::<Value>(&body_text) {
                        Ok(mut json) => {
                            if json.is_object() {
                                if let Some(obj) = json.as_object_mut() {
                                    obj.remove("_notes");
                                }
                                Ok(json.to_string())
                            } else {
                                Ok(body_text)
                            }
                        }
                        Err(_) => Ok(body_text),
                    };
                }
            }
        }
        Err("No token response found".to_string())
    })
    .await;

    // Always clean up browser resources
    let _ = browser.close().await;
    handler_task.abort();

    match token_result {
        Ok(result) => result,
        Err(_) => Err("Timeout waiting for token response".to_string()),
    }
}
