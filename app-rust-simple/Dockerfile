FROM rust:1.85-alpine AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build -vv --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/app-rust-simple .
EXPOSE 8080
CMD ["./app-rust-simple"]
