use std::fmt::Display;

use rocket::data::ByteUnit;

#[derive(Debug, Clone, confique::Config)]
pub struct Conf {
    #[config(nested)]
    pub server: ConfServer,
    #[config(nested)]
    pub db: ConfDB,
    #[config(nested)]
    pub jwt: ConfJWT,
}

#[derive(Debug, Clone, confique::Config)]
pub struct ConfServer {
    #[config(env = "PORT", default = 8080)]
    pub port: u16,
    #[config(env = "ADMIN_API")]
    pub admin_api: Option<String>,
    #[config(env = "ALLOW_NEW_REGISTER", default = true)]
    pub allow_new_register: bool,
    #[config(nested)]
    pub limits: ConfServerLimits,
}

#[derive(Debug, Clone, confique::Config)]
pub struct ConfServerLimits {
    #[config(env = "LIMITS_JSON", default = "4MiB")]
    pub json: ByteUnit,
}

#[derive(Debug, Clone, confique::Config)]
pub struct ConfJWT {
    #[config(env = "JWT_SECRET", default = "")]
    pub secret: String,
    #[config(default = "http://0.0.0.0:8080/")]
    pub issuer: String,
    #[config(default = "http://0.0.0.0:8080/resource")]
    pub audience: String,
}

#[cfg(feature = "sqlite")]
#[derive(Debug, Clone, confique::Config)]
pub struct ConfDB {
    #[config(env = "DATABASE_URL", default = "data.db")]
    pub url: String,
}

#[cfg(feature = "mysql")]
#[derive(Debug, Clone, confique::Config)]
pub struct ConfDB {
    #[config(env = "DATABASE_NAME", default = "kotatsu_db")]
    pub name: String,
    #[config(env = "DATABASE_HOST", default = "localhost")]
    pub host: String,
    #[config(env = "DATABASE_PORT", default = 3306)]
    pub port: u16,
    #[config(env = "DATABASE_USER")]
    pub user: String,
    #[config(env = "DATABASE_PASSWORD")]
    pub password: String,
}

impl ConfDB {
    #[cfg(feature = "sqlite")]
    pub fn url(&self) -> String {
        self.url.clone()
    }

    #[cfg(feature = "mysql")]
    pub fn url(&self) -> String {
        format!(
            "mysql://{user}:{password}@{host}:{port}/{name}",
            user = self.user,
            password = self.password,
            host = self.host,
            port = self.port,
            name = self.name,
        )
    }
}

impl Display for Conf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("Config\n  ")?;

        f.pad("server.port: ")?;
        self.server.port.fmt(f)?;
        f.pad("\n  server.admin_api: ")?;
        if let Some(admin_api) = &self.server.admin_api {
            f.pad(admin_api)?;
        } else {
            f.pad("[empty]")?;
        }
        f.pad("\n  server.allow_new_register: ")?;
        self.server.allow_new_register.fmt(f)?;
        f.pad("\n  server.limits.json: ")?;
        self.server.limits.json.fmt(f)?;

        f.pad("\n  db.url: ")?;
        f.pad(&self.db.url())?;

        f.pad("\n  jwt.secret: ")?;
        if self.jwt.secret.is_empty() {
            f.pad("[empty]")?;
        } else {
            f.pad("[redacted]")?;
        }
        f.pad("\n  jwt.issuer: ")?;
        f.pad(&self.jwt.issuer)?;
        f.pad("\n  jwt.audience: ")?;
        f.pad(&self.jwt.audience)?;

        Ok(())
    }
}
