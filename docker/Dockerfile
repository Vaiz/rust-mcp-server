# sudo docker build -f docker/Dockerfile -t rust-mcp-server .

FROM rust:latest AS builder

RUN apt update && apt install -y musl-tools musl-dev
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY ../Cargo.toml ../Cargo.lock ./
COPY ../src ./src
COPY ../prompts ./prompts
COPY ../docs ./docs

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-mcp-server /usr/bin/rust-mcp-server

USER 1000
ENTRYPOINT ["/usr/bin/rust-mcp-server"]
