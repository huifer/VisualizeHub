use serde::{Deserialize, Serialize};

/// Struct representing MySQL connection information.
#[derive(Debug, Serialize, Deserialize)]
pub struct MysqlUserPassword {
    /// MySQL username.
    pub username: String,

    /// MySQL password.
    pub password: String,

    /// MySQL host (hostname or IP address).
    pub host: String,

    /// MySQL port number.
    pub port: u16,

    /// MySQL database name.
    pub name: String,
}
