FROM rust:1.78-slim as builder
WORKDIR /app
COPY ../../libs ./libs
COPY . .
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/listener /listener
CMD ["/listener"]
