# Build ftl in a seperate build container
FROM rust:alpine AS builder
WORKDIR /usr/src/ftl
RUN apk add libc-dev
COPY . .
RUN cargo install --path .

# Execute ftl in a slim container to reduce image size
FROM alpine
# Copy the binary from the build stage
COPY --from=builder /usr/local/cargo/bin/ /usr/local/bin/
ENTRYPOINT ["fortytwo-lang"]