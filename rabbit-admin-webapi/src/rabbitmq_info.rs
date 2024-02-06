use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RabbitMQInfo {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: i32,
}

impl RabbitMQInfo {
    pub(crate) fn new(username: &str, password: &str, host: &str, port: i32) -> Self {
        RabbitMQInfo {
            username: username.to_string(),
            password: password.to_string(),
            host: host.to_string(),
            port: port,
        }
    }
}
