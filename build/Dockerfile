FROM rust:1.41.0-slim-buster

RUN apt-get update -y && \
    apt-get install -y libssl-dev pkg-config && \
    apt-get clean

WORKDIR /build
