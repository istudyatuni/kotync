[private]
@default:
	just --list

up-mysql:
	docker compose up -d test-db

connect-mysql port="3307" user="root" db="kotatsu_db_test":
	mariadb -h 0.0.0.0 -P {{port}} -u {{user}} {{db}}

test-mysql:
	cargo test --no-default-features --features=original -- --test-threads 1

# drop volume kotync_db_data
remake-mysql: && up-mysql
	docker compose down test-db
	docker volume rm kotync_db_data

test: && test-mysql
	cargo test --no-default-features --features=new

clippy:
	cargo clippy --no-default-features --features=new
	cargo clippy --no-default-features --features=original
