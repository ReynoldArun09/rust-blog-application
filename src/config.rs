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
                .unwrap_or_else(|_| "8000".into())
                .parse()
                .expect("Port must be a valid number"),
            mongodb_uri: env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            database_name: env::var("DATABASE_NAME").unwrap_or_else(|_| "rust_blog".to_string()),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "default_jwt_secret".to_string()),
        }
    }
}
