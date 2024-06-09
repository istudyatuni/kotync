# Kotync

Rewrite of [Kotatsu Synchronization Server](https://github.com/KotatsuApp/kotatsu-syncserver) in Rust.

## Compatibility

- Can work with MySQL database from original server (not fully tested yet). See [Building](/docs/building.md).

## Differences

- Uses Blake3 for hashing passwords
- Configurable via config file

### API differences

- `get /manga`: max `limit` is 1000

## Why?

- To run on low hardware
- To run as single-binary
- To not use database server (SQLite is used instead)

## Installing

See [Building](/docs/building.md).
