FROM rust:1.32

WORKDIR /app
ADD . /app

RUN cargo install diesel_cli
RUN cargo install systemfd
RUN cargo install cargo-watch

