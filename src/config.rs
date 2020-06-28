use rocket::config::{Config, ConfigError, Environment, Value};
use std::env;
use std::collections::HashMap;
use chrono::Duration;

#[derive(Debug)]
pub struct AppConfig {
    pub auth_token_timeout_days: Duration,
    pub cors_allow_origin: String,
    pub cors_allow_methods: String,
    pub cors_allow_headers: String,
    pub environment_name: String,
}

impl Default for AppConfig {
    fn default() -> AppConfig {
        AppConfig {
            auth_token_timeout_days: Duration::days(30),
            cors_allow_origin: String::from("*"),
            cors_allow_methods: String::from("*"),
            cors_allow_headers: String::from("*"),
            environment_name: String::from("unconfigured"),
        }
    }
}

pub fn get_rocket_config() -> Result<(AppConfig, Config), ConfigError> {
    let app_config = AppConfig {
        environment_name: String::from("local"),
        ..Default::default()
    };

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(env::var("DATABASE_URL").unwrap()));
    databases.insert("postgres_db", Value::from(database_config));

    let rocket_config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(5000)
        .extra("databases", databases)
        .finalize()?;

    Ok((app_config, rocket_config))
}
