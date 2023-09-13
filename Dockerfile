# AS builder
FROM registry.hub.docker.com/library/rust:1.72-alpine

WORKDIR /builder
RUN cargo init --bin
COPY Cargo.toml Cargo.lock ./
RUN cargo build --locked --release
COPY ./src ./src
RUN cat ./src/main.rs
RUN cargo install --locked --path .

RUN cp /usr/local/cargo/bin/nix-cache-check /
ENTRYPOINT ["/nix-cache-check"]

# FROM registry.hub.docker.com/library/alpine:3

# WORKDIR /
# COPY --from=builder /usr/local/cargo/bin/nix-cache-check .

# ENTRYPOINT ["/nix-cache-check"]
