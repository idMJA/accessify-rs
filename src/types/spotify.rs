use tokio::time::Instant;

#[derive(Clone)]
pub struct SpotifyToken {
    pub value: String,
    pub expires_at: Instant,
}
