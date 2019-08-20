FROM rust

WORKDIR /app

RUN USER=root cargo init
COPY Cargo.toml .
RUN cargo build --release

RUN cargo install diesel_cli
RUN cargo install systemfd
RUN cargo install cargo-watch

ADD . /app
