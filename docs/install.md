## Running

Some notes:

1. See description of configuration variables in [config.md](config.md)
2. VERSION can be `dev`, or the version itself, like `0.1.0`. See all tags [here](https://github.com/istudyatuni/kotync/pkgs/container/kotync)
3. Version prefixes:
    - Without prefix: server with SQLite as storage
    - `VERSION-mysql`: server with MySQL
    - `VERSION-original`: server with MySQL from original server

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
