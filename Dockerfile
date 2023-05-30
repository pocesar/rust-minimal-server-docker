FROM rust:1.69-alpine as builder

WORKDIR /usr/src/app

RUN apk add --no-cache --update musl-dev
RUN rustup target add x86_64-unknown-linux-musl
COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM busybox:stable-musl

WORKDIR /serve
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/rust-minimal-server-docker /rust-minimal-server-docker
RUN chmod +x /rust-minimal-server-docker

ENTRYPOINT [ "/rust-minimal-server-docker" ]


