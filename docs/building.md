## Installing

You can use pre-built docker image:

```sh
# SQLite
docker run -d -p 8081:8080 \
	-e DATABASE_URL=path_to_your_sqlite_db \
	-e JWT_SECRET=your_secret \
	-e ALLOW_NEW_REGISTER=true \
	--restart always \
	--name kotync ghcr.io/istudyatuni/kotync:dev

# Work on MySQL database from original server
docker run -d -p 8081:8080 \
	-e DATABASE_HOST=your_mysql_db_host \
	-e DATABASE_USER=your_mysql_db_user \
	-e DATABASE_PASSWORD=your_mysql_db_password \
	-e DATABASE_NAME=your_mysql_db_name \
 	-e DATABASE_PORT=your_mysql_db_port \
	-e JWT_SECRET=your_secret \
	-e ALLOW_NEW_REGISTER=true \
	--restart always \
	--name kotync ghcr.io/istudyatuni/kotync:dev-original
```

*todo: check if can connect to locally running mysql*

## Building

### With docker

```sh
# SQLite
docker build github.com/istudyatuni/kotync.git -t kotync

# Work on MySQL database from original server
docker build github.com/istudyatuni/kotync.git -t kotync:mysql --build-arg kind=original
```

### From source

Requires `sqlite` or `mysql` library installed.

```sh
# SQLite
cargo b --release

# Work on MySQL database from original server
cargo b --release --no-default-features --features=original
```

<!-- #### Cross-compile

You need [cross](https://github.com/cross-rs/cross?tab=readme-ov-file#installation) installed.

```sh
# SQLite
cross b --release --target=x86_64-unknown-linux-musl

# MySQL
cargo b --release --no-default-features --features=original --target=x86_64-unknown-linux-musl
``` -->

## Configuring

You can configure everything via `config.toml`. Also you can set some values via `.env` or plain environment variables, see `.env.sample` for available options. Precedence of configuration: env > config.

## Running

### Docker image

Run previously built image:

See command in [Running](#installing), and replace `ghcr.io/..` with `kotync:latest` for SQLite and with `kotync:mysql` for MySQL.

<!-- ### Single binary (systemd)

After building from source,  -->

<!-- You need to [build with `cross`](#with-cross) -->
