# Build stage
FROM rust:slim-stretch as cargoer
RUN rustup default nightly
COPY $HOME/jwt-gang ./jwt-gang
COPY $HOME/mailgang ./mailgang
COPY $HOME/xam-xam-common ./xam-xam-common
COPY $HOME/xam-xam-dal ./xam-xam-dal
COPY $HOME/xam-xam-id-bll ./xam-xam-id-bll
COPY $HOME/xam-xam-id-web ./xam-xam-id-web
WORKDIR /xam-xam-id-web
RUN cargo build --release && strip /xam-xam-id-web/target/release/xam-xam-id-web

# Final stage
FROM debian:stretch-slim
COPY --from=cargoer $HOME/identity_web/target/release/xam-xam-id-web .
COPY xam-xam-id-web/prod.env .env
COPY xam-xam-id-web/Jwt.toml .
EXPOSE 8000
CMD ["./xam-xam-id-web"]