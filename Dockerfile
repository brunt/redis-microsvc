FROM ubuntu:latest

ENV CC=musl-gcc \
    PREFIX=/musl \
    PATH=/usr/local/bin:/root/.cargo/bin:$PATH \
    PKG_CONFIG_PATH=/usr/local/lib/pkgconfig \
    LD_LIBRARY_PATH=$PREFIX \
    CARGO_TARGET_DIR=/

RUN apt-get update && apt-get install -y \
  musl-dev \
  musl-tools \
  file \
  git \
  make \
  g++ \
  curl \
  pkgconf \
  ca-certificates \
  xutils-dev \
  libssl-dev \
  libpq-dev \
  automake \
  autoconf \
  libtool \
  --no-install-recommends && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /rust/redis-microsvc/
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    ~/.cargo/bin/rustup target add x86_64-unknown-linux-musl && \
    echo "[build]\ntarget = \"x86_64-unknown-linux-musl\"" > ~/.cargo/config &&\
    ~/.cargo/bin/cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest
WORKDIR /root/
COPY --from=0 /x86_64-unknown-linux-musl/release/redis-microsvc .
CMD ["./app"]
