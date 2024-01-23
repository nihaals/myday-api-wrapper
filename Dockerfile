FROM rust:slim-bookworm AS builder
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /src/target/release/myday-api-wrapper /server
ENV HOST=0.0.0.0
CMD ["/server"]
