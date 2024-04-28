# ARG BINARY_NAME="pong_game_tcp_socket"

FROM rust:1.76 as build

WORKDIR /app

COPY . .

RUN cargo test
RUN cargo build --release



# FROM alpine:3.19.1 as release

# WORKDIR /usr/local/bin/

# EXPOSE 8000 8080

# COPY --from=build /app/target/release/$BINARY_NAME .

# CMD [ $BINARY_NAME ]
