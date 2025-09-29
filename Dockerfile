# new, original, mysql
ARG kind=new

# ----- build ----- #

FROM rust:1.86-alpine AS builder

# install even unnecessary deps for better caching
RUN apk add --no-cache musl-dev sqlite-static mariadb-dev

# https://github.com/rust-lang/rust/issues/115430
ENV RUSTFLAGS="-Ctarget-feature=-crt-static"

WORKDIR /app
COPY . /app

# https://docs.docker.com/build/cache/optimize/#use-cache-mounts
# CARGO_HOME is defined in https://github.com/rust-lang/docker-rust/blob/master/Dockerfile-alpine.template
# copy at the end because can't tell cargo to put the resulting exe somewhere - https://github.com/rust-lang/cargo/issues/6790
RUN --mount=type=cache,target=/app/target/ \
	--mount=type=cache,target=/usr/local/cargo/git/db/ \
	--mount=type=cache,target=/usr/local/cargo/registry/ \
	cargo b --release --target=x86_64-unknown-linux-musl --no-default-features --features=${kind} \
	&& cp /app/target/x86_64-unknown-linux-musl/release/kotync /kotync

# ----- select dependencies ----- #

# https://stackoverflow.com/a/60820156
FROM alpine:3.20 AS run-base

FROM run-base AS run-new
ENV DEPS=sqlite-libs

FROM run-base AS run-original
ENV DEPS=mariadb-connector-c

FROM run-base AS run-mysql
ENV DEPS=mariadb-connector-c

# ----- result ----- #

FROM run-${kind} AS run
RUN apk add --no-cache libgcc ${DEPS}

FROM run
EXPOSE 8080:8080
WORKDIR /app
COPY --from=builder /kotync .
ENTRYPOINT ["/app/kotync"]
