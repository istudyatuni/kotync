# Changelog

## Unreleased

- RUST_LOG can now be set in `.env`

## v0.3.0-beta.1 (2025-09-28)

### Added

- Server can now be built as a static binary
- JWT issuer can be configured with `JWT_ISSUER` env now

### Updated

- Update to latest changes (up to Kotatsu 9.2)

### Removed

- JWT audience cannot be configured now

## v0.2 (2024-08-16)

### Added

- Configure port (env `PORT`)
- Configure limit for JSON requests payloads (env `LIMITS_JSON`)

### Changed

- Config structure has been changed

## v0.1 (2024-06-01)

Initial release. Includes all features from original server. Also includes:

- Use SQLite for storage
- Use and migrate MySQL database from original server
- Uses Blake3 for hashing passwords
- Can be configured via config file
- `get /manga`: max `limit` is 1000
