# Kotync

Rewrite of [Kotatsu Synchronization Server](https://github.com/KotatsuApp/kotatsu-syncserver) in Rust.

## Configuring

You can configure everything via `config.toml`. Also you can set some values via `.env` or plain environment variables, see `.env.sample` for available options. Precedence of configuration: env > config.

## Differences

- Uses Blake3 for hashing passwords
- Configurable via config file
- Compatibility with MySQL database is planned (including migration of old passwords), though it's not easy. Some work started on `mysql` branch

## Why?

- To run on low hardware
- To run as single-binary
- To not use database server (SQLite is used instead)

## Build from source

```sh
cargo b --release
```
