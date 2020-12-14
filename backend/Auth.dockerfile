# Build stage
FROM rust:slim-stretch as cargoer
COPY jwt-gang/ .
COPY mailgang/ .
COPY xam-xam-common/ .
COPY xam-xam-dal/ .
COPY xam-xam-id-bll/ .
COPY xam-xam-id-web/ .
WORKDIR /xam-xam-id-web
RUN cargo build --release && strip target/release/xam-xam-id-web

# Final stage
FROM debian:stretch-slim
COPY --from=cargoer xam-xam-id-web/target/release/xam-xam-id-web .
ARG ENV_FILE_PATH=xam-xam-id-web/.env
ARG JWT_FILE_PATH=xam-xam-id-web/Jwt.toml
COPY $ENV_FILE_PATH ./env
COPY $JWT_FILE_PATH ./Jwt.toml
EXPOSE 8000
EXPOSE 6379
EXPOSE 5432
CMD ["./xam-xam-id-web"]