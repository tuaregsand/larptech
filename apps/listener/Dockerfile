FROM rust:1.78-slim AS builder
RUN apt-get update && \
    apt-get install -y musl-tools protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY libs ./libs
COPY apps/listener ./apps/listener
WORKDIR /app/apps/listener
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /app/apps/listener/target/x86_64-unknown-linux-musl/release/listener /listener
CMD ["/listener"]
