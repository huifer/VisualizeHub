use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ZkUserPassword {
    pub url: String,

    pub name: String,
}
