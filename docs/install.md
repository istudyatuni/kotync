## Running

Some notes:

1. See description of configuration variables in [config.md](config.md)
1. VERSION can be `dev`, or the version itself, like `0.1.0`. See all tags [here](https://github.com/istudyatuni/kotync/pkgs/container/kotync)
1. Version prefixes:
    - Without prefix: server with SQLite as storage
    - `VERSION-mysql`: server with MySQL
    - `VERSION-original`: server with MySQL that can run on top of database created by the original server
1. "Original" version does not support databases created before [July 31, 2025](https://github.com/KotatsuApp/kotatsu-syncserver/commit/13673dd0d3d40b974062ccc9bf88f1a39dfa909e) (because the database structure was significantly altered in [KotatsuApp/kotatsu-syncserver#25](https://github.com/KotatsuApp/kotatsu-syncserver/pull/25))
1. I think "original" version _should_ currently support newer versions, but I haven't been able to test it

### As a static binary

Since v0.3 static binary for Linux is provided (only "sqlite" version). Download `kotync.tar.gz` from [release assets](https://github.com/istudyatuni/kotync/releases/latest), and then:

```sh
# unpack
tar xzf kotync.tar.gz

# generate random uuid for secret
echo JWT_SECRET=$(uuidgen) > .env
# or set it manually
echo JWT_SECRET=YOUR_SECRET > .env

mv kotync /usr/local/bin

# run (in the same directory where .env file is located)
kotync
```

### With docker-compose

Download [`docker-compose.yaml`](https://github.com/istudyatuni/kotync/blob/master/docs/docker-compose.yml):

```sh
curl -L https://github.com/istudyatuni/kotync/raw/master/docs/docker-compose.yml -o docker-compose.yml
```

After that fill environment variables inside. Then run:

```sh
# SQLite
docker compose up -d server

# MySQL (original or mysql is set in docker-compose)
docker compose up -d server-mysql
```

### With docker

Use pre-built docker image:

```sh
# SQLite
docker run -d -p 8081:8080 \
    -e DATABASE_URL=path_to_your_sqlite_db \
    -e JWT_SECRET=your_secret \
    -e ALLOW_NEW_REGISTER=true \
    --restart always \
    --name kotync ghcr.io/istudyatuni/kotync:VERSION

# MySQL
docker run -d -p 8081:8080 \
    -e DATABASE_HOST=your_mysql_db_host \
    -e DATABASE_USER=your_mysql_db_user \
    -e DATABASE_PASSWORD=your_mysql_db_password \
    -e DATABASE_NAME=your_mysql_db_name \
    -e DATABASE_PORT=your_mysql_db_port \
    -e JWT_SECRET=your_secret \
    -e ALLOW_NEW_REGISTER=true \
    --restart always \
    --name kotync ghcr.io/istudyatuni/kotync:VERSION-original
```

### Note on MySQL

***When using `docker`***

You should allow your mysql user to connect from `172.17.0.2`\*. To do this, connect to mysql under root user (via `sudo mysql`), and execute:

```sql
CREATE USER 'some_user'@'172.17.0.%' IDENTIFIED BY 'some_password';
GRANT ALL PRIVILEGES ON kotatsu_db.* TO 'some_user'@'172.17.0.%';
```

\* IP of server, when it connects to local MySQL from docker network. See IP of docker network: `ip a | grep -A 3 docker`.

After that, set `DATABASE_HOST=172.17.0.1`.

***When using `docker compose`***

MySQL user is created on startup with all permissions, if you want to set more granular permissions, you should allow user to connect from within docker network. To do this, first run `docker compose up -d db`, then connect to mariadb under root user (via `sudo mysql` or `sudo mariadb`), and execute:

```sql
CREATE USER 'some_user'@'172.20.0.3' IDENTIFIED BY 'some_password';
GRANT ALL PRIVILEGES ON kotatsu_db.* TO 'some_user'@'172.20.0.3';
```

See more details [here](https://stackoverflow.com/a/44544841).

## Building

### With docker

```sh
# SQLite
docker build github.com/istudyatuni/kotync.git -t kotync

# MySQL (to build for original DB change mysql to original)
docker build github.com/istudyatuni/kotync.git -t kotync:mysql --build-arg kind=mysql
```

### From source

Requires `sqlite` or `mysql` library installed.

```sh
# SQLite
cargo b --release

# MySQL (to build for original DB change mysql to original)
cargo b --release --no-default-features --features=mysql
```

### From source (static binary)

#### For Linux

```sh
# requires https://github.com/casey/just
just build-static-sqlite

# or manually
docker run --rm -it \
    -v "$(pwd)":/build \
    -w /build \
    ghcr.io/rust-cross/rust-musl-cross:x86_64-musl \
    cargo build --release --features sqlite-bundled --target=x86_64-unknown-linux-musl
```

<!-- #### Cross-compile

You need [cross](https://github.com/cross-rs/cross?tab=readme-ov-file#installation) installed.

```sh
# SQLite
cross b --release --target=x86_64-unknown-linux-musl

# MySQL
cargo b --release --no-default-features --features=mysql --target=x86_64-unknown-linux-musl
``` -->

<!-- ## Running

### Single binary (systemd)

After building from source, -->

<!-- You need to [build with `cross`](#with-cross) -->
