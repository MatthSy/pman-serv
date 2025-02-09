# Stage 1
FROM rust:1.80.1 as builder

WORKDIR /apps/pman
COPY . .

RUN cargo build --release

# Stage 2
FROM debian:stable-slim

RUN apt-get update

WORKDIR /apps/pman
COPY --from=builder /apps/pman/target/release/serv ./main
COPY . .

EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
ENV ROCKET_LOG_LEVEL=debug

CMD ["./main"]
