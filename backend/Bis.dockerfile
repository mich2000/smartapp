# Build stage
FROM rust:slim-stretch as cargoer
COPY jwt-gang/ .
COPY mailgang/ .
COPY xam-xam-common/ .
COPY xam-xam-dal/ .
COPY xam-xam-bis-bll/ .
COPY xam-xam-bis-web/ .
WORKDIR /xam-xam-bis-web
RUN cargo build --release && strip target/release/xam-xam-bis-web

# Final stage
FROM debian:stretch-slim
COPY --from=cargoer xam-xam-bis-web/target/release/xam-xam-bis-web .
ARG ENV_FILE_PATH
ARG JWT_FILE_PATH
COPY $ENV_FILE_PATH ./env
COPY $JWT_FILE_PATH ./Jwt.toml
EXPOSE 8001
EXPOSE 5432
CMD ["./xam-xam-bis-web"]