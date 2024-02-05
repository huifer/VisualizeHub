use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InfluxDBUserPassword {
    pub url: String,
    pub name: String,
    pub auth_token: String,
    pub org: String,
    pub version:String,
}