FROM rust:1.76

WORKDIR /app

COPY . .

EXPOSE 8080

RUN cargo test
RUN cargo build --release