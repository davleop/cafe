FROM rust:latest AS RUST_LATEST
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src/lib.rs ./src/lib.rs
RUN rustup default nightly
RUN cargo fetch
COPY ./src ./src
RUN cargo build --release --bin client
RUN cargo install --path .

FROM debian:latest
WORKDIR /app
RUN apt update && apt install -y ca-certificates && apt install -y iproute2
COPY --from=RUST_LATEST /usr/local/cargo/bin/client /app
CMD ["sh", "-c", "/app/client 1 ' 172.17.0.2'"]

