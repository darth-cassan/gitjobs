# Build server
FROM rust:1-alpine3.21 AS builder
RUN apk --no-cache add musl-dev perl make
WORKDIR /ojb
COPY Cargo.* ./
COPY ojb-server ojb-server
WORKDIR /ojb/ojb-server
RUN cargo build --release

# Final stage
FROM alpine:3.21.2
RUN apk --no-cache add ca-certificates && addgroup -S ojb && adduser -S ojb -G ojb
USER ojb
WORKDIR /home/ojb
COPY --from=builder /ojb/target/release/ojb-server /usr/local/bin
