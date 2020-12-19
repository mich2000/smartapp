FROM debian:stretch-slim
RUN apt-get update && \
apt-get -y install pkg-config libssl-dev libpq-dev && \
echo "/usr/local/lib64" > /etc/ld.so.conf.d/openssl.conf && ldconfig