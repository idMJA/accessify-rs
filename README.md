# ⚠️ WARNING: This project is in testing phase! ⚠️

> This Rust version of Accessify is under development and testing. Features, APIs, and stability may change at any time. Use at your own risk!

# Accessify (Rust versions)

> **Credit:** This project is based on [devoxin/anonify](https://github.com/devoxin/anonify) with refactoring and adaptation for LavaSrc and general Spotify access token needs.

A simple REST API to generate and cache anonymous Spotify access tokens using browser automation (chromiumoxide). Designed for use as a custom anonymous token endpoint for [LavaSrc](https://github.com/topi314/LavaSrc) on Lavalink, but can be used by any service needing a fresh Spotify access token.

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

## Build & Run

### Local (default target)

```bash
cargo build --release
./target/release/accessify-rs
```

### Cross-compile (Linux, ARM, Windows, macOS)

- **Via [cross](https://github.com/cross-rs/cross) (recommended, needs Docker):**
  ```bash
  cross build --release --target x86_64-unknown-linux-gnu      # Linux x86_64
  cross build --release --target aarch64-unknown-linux-gnu     # Linux ARM64
  cross build --release --target x86_64-pc-windows-msvc        # Windows x86_64
  cross build --release --target x86_64-apple-darwin           # macOS x86_64
  cross build --release --target aarch64-apple-darwin          # macOS ARM64
  ```

- **Via GitHub Actions:**  
  Project includes a workflow to build for all major platforms and upload artifacts automatically.

## API Usage

- **GET /spotifytoken**
  - Returns a cached Spotify access token (auto-refresh if expired)

Example:
```bash
curl http://localhost:3000/spotifytoken
```

## Integration: LavaSrc Custom Anonymous Token Endpoint

This API can be used as a custom anonymous token endpoint for [LavaSrc](https://github.com/topi314/LavaSrc) and Lavalink.

**Example LavaSrc on Lavalink config:**
```yaml
spotify:
  preferAnonymousToken: true
  customAnonymousTokenEndpoint: "http://localhost:3000/spotifytoken"
```

## Notes

- For deployment on server/CI, make sure Chromium/Chrome is available and accessible.
- If `CHROME_PATH` is not set, the system will try to use the default browser in PATH.
- Request logs include user-agent and timestamp.
- For ARM or other platforms, see the build matrix in `.github/workflows/build-matrix.yml`.

---
