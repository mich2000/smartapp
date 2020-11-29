# Build stage
FROM rust:slim-stretch as cargoer
RUN rustup default nightly
COPY $HOME/jwt-gang ./jwt-gang
COPY $HOME/mailgang ./mailgang
COPY $HOME/xam-xam-common ./xam-xam-common
COPY $HOME/xam-xam-dal ./xam-xam-dal
COPY $HOME/xam-xam-bis-bll ./xam-xam-bis-bll
COPY $HOME/xam-xam-bis-web ./xam-xam-bis-web
WORKDIR /xam-xam-bis-web
RUN cargo build --release && strip /xam-xam-bis-web/target/release/xam-xam-bis-web

# Final stage
FROM debian:stretch-slim
COPY --from=cargoer $HOME/identity_web/target/release/xam-xam-bis-web .
COPY xam-xam-bis-web/prod.env .env
COPY xam-xam-bis-web/Jwt.toml .
EXPOSE 8001
CMD ["./xam-xam-bis-web"]