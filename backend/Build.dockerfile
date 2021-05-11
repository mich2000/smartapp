FROM rust:slim-stretch as cargoer
RUN apt-get update && apt-get -y install pkg-config libssl-dev libpq-dev && apt-get autoremove

COPY jwt-gang/ ./jwt-gang
COPY xam-xam-common/ ./xam-xam-common
COPY xam-xam-dal/ ./xam-xam-dal
COPY mailgang/ ./mailgang
COPY xam-xam-id-bll/ ./xam-xam-id-bll
COPY xam-xam-id-web/ ./xam-xam-id-web
COPY xam-xam-bis-bll/ ./xam-xam-bis-bll
COPY xam-xam-bis-web/ ./xam-xam-bis-web
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release && strip target/release/xam-xam-id-web && strip target/release/xam-xam-bis-web