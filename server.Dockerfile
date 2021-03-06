FROM rust:latest AS RUST_LATEST
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src/lib.rs ./src/lib.rs
RUN rustup default nightly
RUN cargo fetch
COPY ./src ./src
RUN cargo build --release --bin server
RUN cargo install --path .

FROM debian:latest
EXPOSE 42069
WORKDIR /app
RUN apt update && apt install -y ca-certificates && apt install -y iproute2
COPY --from=RUST_LATEST /usr/local/cargo/bin/server /app
CMD ["sh", "-c", "/app/server"]

