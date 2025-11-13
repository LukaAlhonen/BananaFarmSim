FROM rust:1.90 AS builder

WORKDIR /usr/src/app

RUN apt update && apt install -y musl-tools
RUN rustup target add aarch64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./

COPY integration ./integration
COPY influxdb3client ./influxdb3client
COPY subscriber ./subscriber
COPY publisher ./publisher

RUN cargo build -p publisher --release --target aarch64-unknown-linux-musl

FROM alpine:latest
COPY --from=builder /usr/src/app/target/aarch64-unknown-linux-musl/release/publisher /publisher
ENTRYPOINT [ "/publisher" ]
