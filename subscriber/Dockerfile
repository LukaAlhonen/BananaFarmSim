FROM --platform=$BUILDPLATFORM rust:1.90 AS builder

ARG TARGETARCH
WORKDIR /usr/src/app

RUN apt update && apt install -y pkg-config build-essential

COPY Cargo.toml Cargo.lock ./

# need to copy all workspaces so cargo does not freak out
COPY integration ./integration
COPY influxdb3client ./influxdb3client
COPY subscriber ./subscriber
COPY publisher ./publisher


RUN cargo build -p subscriber --release

FROM alpine:latest
COPY --from=builder /usr/src/app/target/release/subscriber /subscriber
ENTRYPOINT [ "/subscriber" ]
