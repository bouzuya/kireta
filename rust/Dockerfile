FROM rust:1.74.0-bookworm

ENV CARGO_BUILD_TARGET_DIR=/tmp/target
RUN useradd --create-home bouzuya \
  && mkdir -p /home/bouzuya/kireta
RUN apt-get update && apt-get install -y \
  jq \
  mold \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*
RUN rustup component add \
  clippy \
  rustfmt
RUN cargo install \
  cargo-watch
WORKDIR /home/bouzuya/kireta
