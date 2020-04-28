FROM rust

WORKDIR /app

RUN USER=root cargo init
COPY Cargo.toml .
RUN cargo build --release

RUN cargo install diesel_cli
RUN cargo install systemfd
RUN cargo install cargo-watch

ADD . /app

RUN apt-get update -yq \
    && apt-get install curl gnupg -yq \
    && curl -sL https://deb.nodesource.com/setup_12.x | bash \
    && apt-get install nodejs -yq

RUN npm install

