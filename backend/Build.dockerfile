FROM rust:slim-stretch as cargoer
RUN apt-get update && apt-get -y install pkg-config libssl-dev libpq-dev
COPY jwt-gang/ ./jwt-gang
COPY xam-xam-common/ ./xam-xam-common
COPY xam-xam-dal/ ./xam-xam-dal