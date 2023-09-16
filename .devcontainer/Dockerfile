FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends build-essential pkg-config libssl-dev lldb protobuf-compiler\
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*

USER vscode

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

ENV PATH="/home/vscode/.cargo/bin:${PATH}"

RUN rustup update

RUN rustup component add clippy rustfmt rust-src rust-analysis rls 

RUN cargo install cargo-edit cargo-deny cargo-udeps cargo-audit cargo-update