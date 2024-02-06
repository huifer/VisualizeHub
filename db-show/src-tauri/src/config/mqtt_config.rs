use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MqttUserPassword {
    /// Mqtt username.
    pub username: String,

    /// Mqtt password.
    pub password: String,

    /// Mqtt host (hostname or IP address).
    pub host: String,

    /// Mqtt port number.
    pub port: u16,

    /// Mqtt database name.
    pub name: String,
}
