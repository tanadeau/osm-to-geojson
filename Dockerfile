# Stage 1: Build application in release mode
FROM ekidd/rust-musl-builder AS builder
ADD . ./
RUN sudo chown -R rust:rust /home/rust
RUN cargo build --release

# Stage 2: Copy to Alpine Linux container
FROM alpine:latest
ARG APP_NAME=osm-to-geojson
RUN apk --no-cache add ca-certificates
WORKDIR /usr/local/bin
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/${APP_NAME} rust-app
ENTRYPOINT ["/usr/local/bin/rust-app"]
