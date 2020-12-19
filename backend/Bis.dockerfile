#build stage
FROM xam-build as cargoer
COPY mailgang/ ./mailgang
COPY xam-xam-id-bll/ ./xam-xam-id-bll
COPY xam-xam-id-web/ ./xam-xam-id-web
WORKDIR /xam-xam-id-web
RUN cargo update
RUN cargo build --release && strip target/release/xam-xam-id-web

# Final stage
FROM xam-run
COPY --from=cargoer xam-xam-id-web/target/release/xam-xam-id-web .
ARG JWT_FILE_PATH
COPY $JWT_FILE_PATH ./Jwt.toml
EXPOSE 8000
CMD ["./xam-xam-id-web"]