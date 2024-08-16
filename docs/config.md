## Configuration

You can configure everything via plain environment variables, `.env` or `config.toml`. Precedence of configuration: env > config.

| Description                                  | Env                  | Default                        | Note                                        |
|:---------------------------------------------|:---------------------|:-------------------------------|:--------------------------------------------|
| Server port                                  | `PORT`               | 8080                           | Do not change when using docker compose     |
| Prefix for admin API<sup>1</sup>             | `ADMIN_API`          | -                              | Optional. If not provided, API is disabled  |
| If should allow new registers                | `ALLOW_NEW_REGISTER` | `true`                         |                                             |
| Limit for JSON payload for requests          | `LIMITS_JSON`        | 4MiB<sup>2</sup>               | Original server has no limit                |
| Secret text for encoding/decoding JWT tokens | `JWT_SECRET`         | -                              | Required                                    |
| JWT issuer                                   | -                    | `http://0.0.0.0:8080/`         |                                             |
| JWT audience                                 | -                    | `http://0.0.0.0:8080/resource` |                                             |
| Path to database file                        | `DATABASE_URL`       | data.db                        | For SQLite                                  |
| Name of database in MySQL                    | `DATABASE_NAME`      | kotatsu_db                     | For MySQL                                   |
| Host of MySQL database                       | `DATABASE_HOST`      | localhost                      | For MySQL                                   |
| Port of MySQL database                       | `DATABASE_PORT`      | 3306                           | For MySQL                                   |
| User for connecting to MySQL database        | `DATABASE_USER`      | -                              | For MySQL. Required                         |
| Password for user in MySQL database          | `DATABASE_PASSWORD`  | -                              | For MySQL. Required                         |
| Log level                                    | `RUST_LOG`           | `error`<sup>3</sup>            | Only plain environment variable (no `.env`) |

1. Enables some additiional features, like statistics. For `/admin` URL will look like `http://IP/admin/stats`
1. Examples: 256 kB, 0.500 mib, 1MB, 1GiB
1. Possible values: `off`, `error`, `warn`, `info`, `debug`, `trace`. In debug build default `info`

### Example `.env`

```bash
# mysql
DATABASE_NAME=kotatsu_db
DATABASE_HOST=localhost
DATABASE_PORT=3306
DATABASE_USER=YOUR_USER
DATABASE_PASSWORD=YOUR_PASSWORD

# sqlite
DATABASE_URL=data.db

PORT=8080
JWT_SECRET=SECRET
ALLOW_NEW_REGISTER=true
ADMIN_API=/admin
LIMITS_JSON=4MiB
```

### Example `config.toml`

Currently name of config file can't be configured.

```toml
[server]
port = 8080
admin_api = "/admin"
allow_new_register = true

[server.limits]
json = "4MiB"

[db]
# sqlite
url = "data.db"

# mysql
name = "kotatsu_db"
host = "localhost"
port = 3306
user = "YOUR_USER"
password = "YOUR_PASSWORD"

[jwt]
secret = ""
issuer = "http://0.0.0.0:8080/"
audience = "http://0.0.0.0:8080/resource"
```
