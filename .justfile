[private]
@default:
	just --list

connect-mysql:
	mariadb -h 0.0.0.0 -P 3307 -u root kotatsu_db_test

up-mysql:
	docker compose up -d test-db

test-mysql:
	cargo test -- --test-threads 1
