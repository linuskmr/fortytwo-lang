# Build ftl in a seperate build container
FROM rust as builder
WORKDIR /usr/src/ftl
COPY . .
RUN cargo install --path .

# Execute ftl in a slim container to reduce image size
FROM debian:buster-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
# Copy the binary from the build stage
COPY --from=builder /usr/local/cargo/bin/ftl /usr/local/bin/ftl
