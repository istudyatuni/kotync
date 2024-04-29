use std::fmt::Display;

#[derive(Debug, Clone, confique::Config)]
pub struct Conf {
    #[config(nested)]
    pub db: ConfDB,
    #[config(nested)]
    pub jwt: ConfJWT,
    #[config(env = "ALLOW_NEW_REGISTER", default = true)]
    pub allow_new_register: bool,
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

#[derive(Debug, Clone, confique::Config)]
pub struct ConfDB {
    #[config(env = "DATABASE_URL", default = "data.db")]
    pub url: String,
}

impl Display for Conf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("Config")?;
        f.pad("\n  db.url: ")?;
        f.pad(&self.db.url)?;
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
        f.pad("\n  allow_new_register: ")?;
        self.allow_new_register.fmt(f)?;

        Ok(())
    }
}
