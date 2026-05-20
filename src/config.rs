use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub app_name: String,
    pub port: u16,
    pub mongodb_uri: String,
    pub database_name: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            app_name: env::var("APP_NAME").unwrap_or_else(|_| "Rust Blog Api".into()),
            port: env::var("PORT")
                .unwrap_or_else(|| "8000".into())
                .parse()
                .expect("Port must be a valid number"),
            mongodb_uri: env::var("MONGODB_URI").expect("Mongo uri is required"),
            database_name: env::var("DATABASE_NAME").expect("Database name is required".into()),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET is required"),
        }
    }
}
