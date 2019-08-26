FROM rust:1.37.0

WORKDIR /app
COPY . ./

RUN cargo build
COPY . ./