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
