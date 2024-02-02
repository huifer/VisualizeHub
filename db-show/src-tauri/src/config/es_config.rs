use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct ESUserPassword {
    /// ES username.
    pub username: String,

    /// ES password.
    pub password: String,

    /// ES host (hostname or IP address).
    pub host: String,

    /// ES port number.
    pub port: u16,

    /// ES database name.
    pub name: String,
}