FROM rust:1-stretch as rust-build-env

WORKDIR /src
RUN USER=root cargo init
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN cargo install diesel_cli --no-default-features --features "postgres"
COPY src src
RUN cargo build --release

FROM debian:stretch-slim
COPY --from=rust-build-env /usr/local/cargo/bin/diesel /usr/local/bin/
COPY --from=rust-build-env /src/target/release/vote /usr/local/bin/
WORKDIR /app
COPY diesel.toml wait-for-it.sh ./
COPY src/schema.rs ./src/
COPY migrations ./migrations
RUN chmod +x wait-for-it.sh && \
    apt-get update && \
    apt-get install -y libpq-dev && \
    apt-get clean autoclean && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt /var/lib/dpkg /var/lib/cache /var/lib/log && \
    touch Cargo.toml