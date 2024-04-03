FROM rust:1.77-slim as builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/oblivio_loco_be-cli /usr/app/oblivio_loco_be-cli

ENTRYPOINT ["/usr/app/oblivio_loco_be-cli"]