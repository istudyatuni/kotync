# new, original
ARG kind=new

# ----- build ----- #

# https://stackoverflow.com/a/60820156
FROM rust:1.78-alpine AS builder-base

FROM builder-base AS builder-new
ENV FEATURES=new

FROM builder-base AS builder-original
ENV FEATURES=original

FROM builder-${kind} AS builder

# install even unnecessary deps for better caching
# it doesn't help for github action, sad
RUN apk add --no-cache musl-dev sqlite-static mariadb-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock /app/

RUN mkdir src && touch src/lib.rs

# https://github.com/rust-lang/rust/issues/115430
ENV RUSTFLAGS="-Ctarget-feature=-crt-static"
ENV BUILD_ARGS="--release --target=x86_64-unknown-linux-musl --no-default-features --features=${FEATURES}"

RUN cargo b ${BUILD_ARGS}

COPY src /app/src
COPY migrations /app/migrations

RUN cargo b ${BUILD_ARGS}

# ----- result ----- #

FROM alpine:3.20 AS run-base

FROM run-base AS run-new
ENV DEPS=sqlite-libs

FROM run-base AS run-original
ENV DEPS=mariadb-connector-c

FROM run-${kind}
RUN apk add --no-cache libgcc ${DEPS}

WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/kotync .

EXPOSE 8080:8080
ENTRYPOINT ["/app/kotync"]
