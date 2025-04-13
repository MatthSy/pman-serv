# Stage 1
FROM rust:latest

RUN apt update

WORKDIR /apps/pman
COPY . .

RUN cargo build --release

COPY ./config.toml ./config.toml
COPY ./security /app/pman/security
COPY ./data /app/pman/data


EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
ENV ROCKET_LOG_LEVEL=debug


CMD ["./target/release/serv"]
