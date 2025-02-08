# Stage 1
FROM rust:1.80.1 as builder

WORKDIR /usr/src/pman/serv
COPY . .

RUN cargo build --release

# Stage 2
FROM debian:buster-slim

WORKDIR /usr/src/pman/serv
COPY --from=builder /usr/src/pman/serv/target/release/serv ./main

EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
ENV ROCKET_LOG_LEVEL=debug

CMD ["./main"]
