#build stage
FROM rust:slim-stretch as cargoer
COPY jwt-gang/ ./jwt-gang
COPY mailgang/ ./mailgang
COPY xam-xam-common/ ./xam-xam-common
COPY xam-xam-dal/ ./xam-xam-dal
COPY xam-xam-id-bll/ ./xam-xam-id-bll
COPY xam-xam-id-web/ ./xam-xam-id-web
WORKDIR /xam-xam-id-web
RUN apt-get update && apt-get -y install pkg-config libssl-dev libpq-dev
RUN cargo update
RUN cargo build --release && strip target/release/xam-xam-id-web

# Final stage
FROM debian:stretch-slim
COPY --from=cargoer xam-xam-id-web/target/release/xam-xam-id-web .
ARG JWT_FILE_PATH
COPY $JWT_FILE_PATH ./Jwt.toml
RUN apt-get update && apt-get -y install pkg-config libssl-dev libpq-dev
RUN echo "/usr/local/lib64" > /etc/ld.so.conf.d/openssl.conf && ldconfig
EXPOSE 8000
CMD ["./xam-xam-id-web"]