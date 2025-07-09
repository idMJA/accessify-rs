
FROM rust:1.88 as builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM linuxserver/chromium:latest
COPY --from=builder /build/target/release/accessify /bin/accessify

ENTRYPOINT ["/bin/accessify"]