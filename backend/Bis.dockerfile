#build stage
FROM xam-build as cargoer
COPY mailgang/ ./mailgang
COPY xam-xam-bis-bll/ ./xam-xam-bis-bll
COPY xam-xam-bis-web/ ./xam-xam-bis-web
WORKDIR /xam-xam-bis-web
RUN cargo update
RUN cargo build --release && strip target/release/xam-xam-bis-web

# Final stage
FROM xam-run
COPY --from=cargoer xam-xam-bis-web/target/release/xam-xam-bis-web .
ARG JWT_FILE_PATH
COPY $JWT_FILE_PATH ./Jwt.toml
EXPOSE 8000
CMD ["./xam-xam-bis-web"]