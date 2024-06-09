## Running

### With docker-compose

Download [`docker-compose.yaml`](https://github.com/istudyatuni/kotync/blob/master/docs/docker-compose.yml):

```sh
curl -L https://github.com/istudyatuni/kotync/raw/master/docs/docker-compose.yml -o docker-compose.yml
```

After that fill environment variables inside, and run:

```sh
# SQLite
docker compose up -d server

# Work on MySQL database from original server
docker compose up -d server-original
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
    --name kotync ghcr.io/istudyatuni/kotync:dev

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
    --name kotync ghcr.io/istudyatuni/kotync:dev-original
```

#### Note on MySQL

When running with docker (not docker compose), you should allow your mysql user to connect from `172.17.0.2`\*. To do this, you can run (via `mysql` cli under root user):

```sql
CREATE USER 'some_user'@'172.17.0.%' IDENTIFIED BY 'some_password';
GRANT ALL PRIVILEGES ON kotatsu_db.* TO 'some_user'@'172.17.0.%';
```

\* IP of server, when it connects to local MySQL from docker network. See IP of docker network: `ip a | grep -A 3 docker`.

After that, set `DATABASE_HOST=172.17.0.1`.

See more details [here](https://stackoverflow.com/a/44544841).

## Building

### With docker

```sh
# SQLite
docker build github.com/istudyatuni/kotync.git -t kotync

# MySQL
docker build github.com/istudyatuni/kotync.git -t kotync:mysql --build-arg kind=original
```

### From source

Requires `sqlite` or `mysql` library installed.

```sh
# SQLite
cargo b --release

# MySQL
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

<!-- ## Running

### Single binary (systemd)

After building from source, -->

<!-- You need to [build with `cross`](#with-cross) -->
