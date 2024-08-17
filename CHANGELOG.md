# Changelog

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
