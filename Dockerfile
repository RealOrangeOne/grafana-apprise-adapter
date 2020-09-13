FROM rust:1-slim as builder

WORKDIR /usr/src/grafana-apprise-adapter

RUN apk --no-cache add ca-certificates openssl-dev

COPY ./src /usr/src/grafana-apprise-adapter/src
COPY Cargo.toml /usr/src/grafana-apprise-adapter/Cargo.toml
COPY Cargo.lock /usr/src/grafana-apprise-adapter/Cargo.lock

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /usr/src/grafana-apprise-adapter/target/release/grafana-apprise-adapter /usr/local/bin/grafana-apprise-adapter

CMD ["/usr/local/bin/grafana-apprise-adapter"]
