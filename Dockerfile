FROM rust:1.79.0 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm

# todo replace with libpq5 for release
RUN apt-get update && apt-get install -y libpq-dev 

WORKDIR /app

COPY --from=builder /app/config.json /app/
COPY --from=builder /app/target/release/rag-api /app/

RUN chmod +x /app/rag-api

CMD ["./rag-api"]
