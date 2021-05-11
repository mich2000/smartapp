FROM xam-run
COPY --from=xam-build target/release/xam-xam-bis-web .
EXPOSE 8000
CMD ["./xam-xam-bis-web"]