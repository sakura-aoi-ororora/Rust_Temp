FROM rust:1.69-bookworm

RUN apt-get update && \
    apt-get -y --no-install-recommends install git && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rustup component add rust-analysis rust-src rustfmt clippy