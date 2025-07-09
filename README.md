# Accessify (Rust version)

> **Original:** This project is a Rust port of [accessify](https://github.com/idMJA/accessify).

A simple REST API to generate and cache anonymous Spotify access tokens using [chromiumoxide](https://github.com/mattsse/chromiumoxide). This project is primarily designed for use as a custom anonymous token endpoint for [LavaSrc](https://github.com/topi314/LavaSrc) on Lavalink, but can be used in any application or service that needs a fresh Spotify access token.

**For the simplest and quickest deployment, or if you are using Pterodactyl, I suggest using the [TypeScript version](https://github.com/idMJA/accessify).**

## Features

- Generate anonymous Spotify access tokens (browser automation, Chromium)
- Designed for seamless integration with LavaSrc/Lavalink (`customAnonymousTokenEndpoint`)
- Token caching with auto-refresh and concurrency-safe (Semaphore)
- Fast REST API (Axum, async)
- Modular, maintainable Rust codebase
- Multi-platform: Linux, Windows, macOS, ARM

## Requirements

- Rust (stable)
- Chromium or Chrome installed and available in PATH, or set `CHROME_PATH` environment variable
- [chromiumoxide](https://github.com/mattsse/chromiumoxide) dependencies (see below)

## Chromium Installation

- **Linux:**  
  ```bash
  sudo apt install chromium-browser
  ```
- **Windows:**  
  Install Chrome/Chromium and ensure it's in your PATH, or set `CHROME_PATH` to the binary.
- **macOS:**  
  Install Chrome or Chromium.

## Build

```bash
cargo build --release
```

### Cross-compile

- **Via [cross](https://github.com/cross-rs/cross) (recommended, needs Docker):**
  ```bash
  cross build --release --target x86_64-unknown-linux-gnu      # Linux x86_64
  cross build --release --target aarch64-unknown-linux-gnu     # Linux ARM64
  cross build --release --target x86_64-pc-windows-msvc        # Windows x86_64
  cross build --release --target x86_64-apple-darwin           # macOS x86_64
  cross build --release --target aarch64-apple-darwin          # macOS ARM64
  ```

### Build Output

- **Linux:** `target/release/accessify-rs`
- **Windows:** `target\release\accessify-rs.exe`

### Run

- **Linux/macOS:**  
  `./target/release/accessify-rs`
- **Windows:**  
  `.\\target\\release\\accessify-rs.exe`
.\target\release\accessify-rs.exe

## API Usage

- **GET /spotifytoken**
  - Returns a cached Spotify access token (auto-refresh if expired)

Example:
```bash
curl http://localhost:3000/spotifytoken
```

## Integration: LavaSrc Custom Anonymous Token Endpoint

This API can be used as a custom anonymous token endpoint for [LavaSrc](https://github.com/topi314/LavaSrc) and Lavalink (see [LavaSrc PR #286](https://github.com/topi314/LavaSrc/pull/286)).

**Example LavaSrc on Lavalink config:**
```yaml
spotify:
  preferAnonymousToken: true
  customAnonymousTokenEndpoint: "http://localhost:3000/spotifytoken"
```

## Notes

- For deployment on server/CI, make sure Chromium/Chrome is available and accessible.
- If `CHROME_PATH` is not set, the system will try to use the default browser in PATH.
- Request logs include IP and user-agent.

---