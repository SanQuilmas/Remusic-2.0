FROM rust:1.86 as builder

WORKDIR /app

COPY . .

RUN cargo remove rdkafka && cargo add rdkafka && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \ 
    apt-get install -y libssl3 librdkafka-dev && \ 
    apt-get clean && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/remusic-back /usr/local/bin/app

EXPOSE 8080

CMD ["app"]