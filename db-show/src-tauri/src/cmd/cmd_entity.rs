use std::collections::{BTreeMap, HashMap};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetMysqlTablesParam {
    pub id: String,
    pub db_name: String,

}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetMysqlTablesCreatedParam {
    pub id: String,
    pub table_name: String,
    pub db_name: String,

}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetRedisInfoParam {
    pub id: String,

}


#[derive(Deserialize, Serialize, Debug)]
pub struct GetRedisKeysParam {
    pub id: String,
    pub db_index: i32,
    pub page: usize,
    pub page_size: usize,

}


#[derive(Deserialize, Serialize, Debug)]
pub struct GetRedisValueParam {
    pub key_type: String,
    pub key_name: String,
    pub db_config_id: String,
    pub db_index: i32,

    #[serde(default)]
    pub expiration_seconds: i32,

}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetRedisHashParam {
    pub key_type: String,
    pub key_name: String,
    pub db_config_id: String,
    pub db_index: i32,

    #[serde(default)]
    pub old_field_values: HashMap<String, String>,
    #[serde(default)]
    pub new_field_values: HashMap<String, String>,

}


#[derive(Deserialize, Serialize, Debug)]
pub struct ChangeRedisSetValueParam {
    pub key_type: String,
    pub key_name: String,
    pub db_config_id: String,

    pub db_index: i32,

    pub old: String,
    #[serde(default)]
    pub new_val: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChangeRedisListValueParam {
    pub key_type: String,
    pub key_name: String,
    pub db_config_id: String,

    pub db_index: i32,

    pub old: String,
    #[serde(default)]
    pub new_val: String,

}


#[derive(Deserialize, Serialize, Debug)]
pub struct SetRedisValueParam {
    pub key_type: String,
    pub key_name: String,
    pub db_config_id: String,
    pub db_index: i32,

    pub value: String,

}


#[derive(Deserialize, Serialize, Debug)]
pub struct ChangeRedisZSetValueParam {
    pub key_type: String,
    pub key_name: String,
    pub db_config_id: String,

    pub db_index: i32,

    pub members_scores: BTreeMap<String, f64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetMongoInfoParam {
    pub id: String,

}


#[derive(Deserialize, Serialize, Debug)]
pub struct SetZookeeperDataParam {
    pub id: String,
    pub path: String,
    pub string_data: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct GetZookeeperChildrenParam {
    pub id: String,
    pub path: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetZookeeperDataParam {
    pub id: String,
    pub path: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateZookeeperDataParam {
    pub id: String,
    pub path: String,
    pub string_data: String,

}

