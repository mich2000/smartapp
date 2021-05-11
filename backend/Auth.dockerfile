FROM xam-run
COPY --from=xam-build target/release/xam-xam-id-web .
EXPOSE 8000
CMD ["./xam-xam-id-web"]