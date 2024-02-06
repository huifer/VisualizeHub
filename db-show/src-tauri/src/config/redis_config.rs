use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisUserPassword {
    /// redis username.
    pub username: String,

    /// redis password.
    pub password: String,

    /// redis host (hostname or IP address).
    pub host: String,

    /// redis port number.
    pub port: u16,

    /// redis database name.
    pub name: String,
}
