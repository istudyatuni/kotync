# Kotync

Rewrite of [Kotatsu Synchronization Server](https://github.com/KotatsuApp/kotatsu-syncserver) in Rust.

## Compatibility

- Can work with MySQL database from original server. See [Building](/docs/install.md).

## Differences

- Uses Blake3 for hashing passwords
- Configurable via config file

### API differences

- `get /manga`: max `limit` is 1000

## Why?

- To run on low hardware
- To run as single-binary (turns out, that's currently problematic, because of linking error during cross-compilation)
- To not use database server (SQLite is used instead by default)

## Installing

See [Building](/docs/install.md).
