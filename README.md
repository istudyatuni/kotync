# Kotync

Rewrite of [Kotatsu Synchronization Server](https://github.com/KotatsuApp/kotatsu-syncserver) in Rust.

## Hosted servers

These are not official servers, use at your own risk.

|Server|Location|Status|
|-|-|-|
|`kosync.estebiu.com`|Nice, France|![](https://img.shields.io/website?url=https%3A%2F%2Fkosync.estebiu.com&label=sync)|
|`kotync.013666.xyz`|San Jose, Usa|![](https://img.shields.io/website?url=https%3A%2F%2Fkotync.013666.xyz&label=sync)|

## Compatibility

- Can work with MySQL database from original server. See [Installing](/docs/install.md).

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

See [Installing](/docs/install.md).
