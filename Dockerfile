FROM rust:1.88-bullseye

RUN cargo install cargo-watch

WORKDIR /app

COPY . .
