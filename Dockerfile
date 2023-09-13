# AS builder
FROM registry.hub.docker.com/library/rust:1.72-alpine

WORKDIR /builder
RUN cargo init --bin
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked
COPY ./src ./src
RUN cargo install --locked --path .
RUN cp $(which nix-cache-check) /bin/

ENTRYPOINT ["cargo", "run"]

# FROM registry.hub.docker.com/library/alpine:3

# WORKDIR /
# COPY --from=builder /usr/local/cargo/bin/nix-cache-check .

# ENTRYPOINT ["/nix-cache-check"]
