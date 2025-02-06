FROM rust:1.80.1

WORKDIR /usr/src/pman/serv
COPY . .

RUN cargo install --path .

CMD ["serv"]
