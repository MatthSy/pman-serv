# Stage 1
FROM rust:1.80.1 AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /apps/pman
COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

# Stage 2
FROM alpine:latest

WORKDIR /apps/pman
RUN adduser -D admin
COPY --from=builder /apps/pman/target/x86_64-unknown-linux-musl/release/serv ./main
COPY ./config.toml ./config.toml
COPY ./security /app/pman/security
COPY ./data /app/pman/data

RUN chown -R admin:admin /app/pman/security
RUN chmod -R a=rw /app/pman/security
USER admin

EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
ENV ROCKET_LOG_LEVEL=debug


CMD ["./main"]
