FROM rust:1.86 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3

COPY --from=builder /app/target/release/remusic-back /usr/local/bin/app

EXPOSE 8080

CMD ["app"]