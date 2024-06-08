# Kotync

Rewrite of [Kotatsu Synchronization Server](https://github.com/KotatsuApp/kotatsu-syncserver) in Rust.

## Configuring

You can configure everything via `config.toml`. Also you can set some values via `.env` or plain environment variables, see `.env.sample` for available options. Precedence of configuration: env > config.

## Differences

- Uses Blake3 for hashing passwords
- Configurable via config file

## Compatibility

- Can work with MySQL database from original server (not fully tested yet). See [Building](/docs/building.md)

### API differences

- `get /manga`: max `limit` is 1000

## Why?

- To run on low hardware
- To run as single-binary
- To not use database server (SQLite is used instead)

## Installing

See [Building](/docs/building.md)
