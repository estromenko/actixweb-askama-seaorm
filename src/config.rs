pub struct Config {
    pub address: String,
    pub db_dsn: String,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            address: std::env::var("ADDRESS").unwrap_or("127.0.0.1:3000".to_string()),
            db_dsn: std::env::var("DB_DSN")
                .unwrap_or("postgres://postgres:secret@127.0.0.1:5432/postgres".to_string()),
        }
    }
}
