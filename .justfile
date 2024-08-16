[private]
@default:
	just --list

up-mysql:
	docker compose up -d test-db

connect-mysql port="3307" user="root" db="kotatsu_db_test":
	mariadb -h 0.0.0.0 -P {{port}} -u {{user}} {{db}}

# drop volume kotync_db_data
remake-mysql: && up-mysql
	docker compose down test-db
	docker volume rm kotync_db_data

[private]
test-new:
	cargo test --no-default-features --features=new

[private]
test-mysql:
	cargo test --no-default-features --features=original -- --test-threads 1
	cargo test --no-default-features --features=mysql -- --test-threads 1

test: test-new test-mysql

clippy:
	cargo clippy --no-default-features --features=new
	cargo clippy --no-default-features --features=mysql
	cargo clippy --no-default-features --features=original

# run checks
check: clippy test

# run checks in CI (no mysql)
ci: && clippy test-new
	cargo fmt --check

format:
	cargo fmt
