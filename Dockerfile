FROM rust:1.76.0-slim-buster as builder

RUN apt-get update && apt-get install libpq-dev -y

WORKDIR /api
COPY . .

# RUN cargo install --path .
RUN cargo build --release

FROM debian:buster-slim as runner

RUN apt-get update && apt-get install libpq-dev -y

COPY --from=builder /api/target/release/api-onlyone-admin /usr/local/bin/api-onlyone-admin
# COPY --from=builder /usr/local/cargo/bin/api-onlyone-admin /usr/local/bin/api-onlyone-admin
WORKDIR /usr/local/bin
CMD ["api-onlyone-admin"]
