use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct ZkUserPassword {
    /// Zk username.
    pub username: String,

    /// Zk password.
    pub password: String,

    /// Zk host (hostname or IP address).
    pub host: String,

    /// Zk port number.
    pub port: u16,

    /// Zk database name.
    pub name: String,
}