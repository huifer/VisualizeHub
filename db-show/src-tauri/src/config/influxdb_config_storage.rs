use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::influxdb_config::InfluxDBUserPassword;
use crate::env::init_file::{get_home_directory, HOME_NAME, INFLUXDB_CONFIG_JSON, read_file_content, write_string_to_file};

#[derive(Debug, Serialize, Deserialize)]
pub struct InfluxDBStorageEntity {
    pub id:String,
    pub url: String,
    pub auth_token: String,
    pub org: String,
    pub version: String,

    /// InfluxDB database name.
    pub name: String,
}

pub trait InfluxDBStorageService {
    fn delete(&mut self, id: String);
    fn write_config_json(&self);
    fn add_username_password_model(&mut self, param: InfluxDBUserPassword);
    fn by_id(&self, id: &str) -> Option<&InfluxDBStorageEntity>;
}

/// 账号密码模式
static USER_PASSWORD_MODEL: i32 = 1;

pub struct InfluxDBStorageManager {
    pub values: Vec<InfluxDBStorageEntity>,

}

impl InfluxDBStorageManager {
    pub fn new() -> Self {
        InfluxDBStorageManager { values: read_redis_json() }
    }
}

impl InfluxDBStorageService for InfluxDBStorageManager {
    fn delete(&mut self, id: String) {
        if let Some(index) = self.values.iter().position(|env| env.id == id) {
            // Remove the environment at the found index
            self.values.remove(index);
            self.write_config_json();
        } else {
            // Handle the case where the environment with the specified alias is not found
            println!("找不到id with id '{}' not found.", id);
        }
    }
    fn write_config_json(&self) {
        let json = serde_json::to_string(&self.values).expect("Failed to serialize to JSON");
        write_redis_json(json);
    }

    fn add_username_password_model(&mut self, param: InfluxDBUserPassword) {
        let uid = Uuid::new_v4();

        let v = InfluxDBStorageEntity {
            id: uid.to_string(),
            url: param.url,
            auth_token: param.auth_token,
            org: param.org,
            version: param.version,
            name: param.name,
        };
        self.values.push(v);
        self.write_config_json();
    }

    fn by_id(&self, id: &str) -> Option<&InfluxDBStorageEntity> {
        self.values.iter().find(|entity| entity.id == id)
    }
}


pub fn write_redis_json(data: String) {
    if let Ok(home_directory) = get_home_directory().ok_or("无法获取 HOME 目录") {
        write_string_to_file(format!("{}/{}/{}", home_directory, HOME_NAME, INFLUXDB_CONFIG_JSON).as_str(), data.as_str()).expect("写入InfluxDB-json失败");
    }
}

pub fn read_redis_json() -> Vec<InfluxDBStorageEntity> {
    if let Ok(home_directory) = get_home_directory().ok_or("无法获取 HOME 目录") {
        let config_path = format!("{}/{}/{}", home_directory, HOME_NAME, INFLUXDB_CONFIG_JSON);
        if let Ok(file_contents) = read_file_content(config_path.as_str()) {
            if let Ok(parsed_data) = serde_json::from_str::<Vec<InfluxDBStorageEntity>>(&file_contents) {
                return parsed_data;
            } else {
                eprintln!("Error parsing JSON data from the config file");
            }
        } else {
            eprintln!("Error reading file content from the config file");
        }
    };
    Vec::new() // Return an empty vector if there's any error
}
