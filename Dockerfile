FROM rust:bookworm AS builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/rmq-listener /app/

CMD ["./rmq-listener"]
