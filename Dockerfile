FROM rust:1.79-alpine3.20 as builder
LABEL authors="tapnisu"

RUN apk update \
    && apk upgrade --available \
    && apk add --no-cache alpine-sdk libressl-dev

WORKDIR /usr/src/shimarin
COPY . .
RUN cargo build --release

FROM alpine:3.20 as runner

RUN apk update \
    && apk upgrade --available \
    && apk add --no-cache ca-certificates \
    && update-ca-certificates

COPY --from=builder /usr/src/shimarin/target/release/shimarin /usr/local/bin/shimarin

CMD ["shimarin"]
