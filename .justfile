[private]
@default:
	just --list

[private]
cargo cmd kind *args:
	cargo {{ cmd }} --no-default-features --features={{ kind }} -- {{ args }}

# start mariadb docker container
up-mysql:
	docker compose up -d test-db

# connect to mariadb docker container
connect-mysql port="3307" user="root" db="kotatsu_db_test":
	mariadb -h 0.0.0.0 -P {{port}} -u {{user}} {{db}}

# drop volume kotync_db_data and restart mariadb container
remake-mysql: && up-mysql
	docker compose down test-db
	docker volume rm kotync_db_data

# create new migration
new-migration name:
	diesel migration generate "{{ name }}"

# generate schema for new migrations
gen-db-schema:
	diesel migration run

# run server
run kind="new": (cargo "run" "new")

[private]
test-new: (cargo "test" "new")

[private]
test-mysql: \
	(cargo "test" "original" "--test-threads" "1") \
	(cargo "test" "mysql" "--test-threads" "1")

# run all tests
test: test-new test-mysql

# run clippy for all variants
clippy *args: \
	(cargo "clippy" "new" args) \
	(cargo "clippy" "mysql" args) \
	(cargo "clippy" "original" args)

# run full checks
check: clippy test

# run checks in CI (no mysql)
ci: && (clippy "-D" "warnings") test-new
	cargo fmt --check

format:
	cargo fmt

shell:
	nix develop --profile flake.drv

# build static binary
build-static-sqlite: && pack-static-sqlite
	@# CARGO_HOME and /tmp/.cargo is used to use local cargo download cache
	docker run --rm -it \
		-v "$(pwd)":/build \
		-v "$HOME/.cargo":/tmp/.cargo \
		-w /build \
		--env=CARGO_HOME=/tmp/.cargo \
		ghcr.io/rust-cross/rust-musl-cross:x86_64-musl \
		cargo build --release \
			--features sqlite-bundled \
			--target=x86_64-unknown-linux-musl \
			--config build.rustc-wrapper="''"

[private]
pack-static-sqlite:
	tar -C target/x86_64-unknown-linux-musl/release -czf target/kotync.tar.gz kotync
