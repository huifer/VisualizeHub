use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MongoUserPassword {
    /// Mongo username.
    pub username: String,

    /// Mongo password.
    pub password: String,

    /// Mongo host (hostname or IP address).
    pub host: String,

    /// Mongo port number.
    pub port: u16,

    /// Mongo database name.
    pub name: String,
}
