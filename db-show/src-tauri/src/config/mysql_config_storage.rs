use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::mysql_config::MysqlUserPassword;
use crate::env::init_file::{
    get_home_directory, read_file_content, write_string_to_file, HOME_NAME, MYSQL_CONFIG_JSON,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct MySQLStorageEntity {
    /// 1. 账号密码模式
    /// 2. ssl
    pub r#type: i32,
    pub id: String,
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

pub trait MysqlStorageService {
    fn delete(&mut self, id: String);
    fn write_config_json(&self);
    fn add_username_password_model(&mut self, param: MysqlUserPassword);
    fn by_id(&self, id: &str) -> Option<&MySQLStorageEntity>;
}

/// 账号密码模式
static USER_PASSWORD_MODEL: i32 = 1;

pub struct MySQLStorageManager {
    pub values: Vec<MySQLStorageEntity>,
}

impl MySQLStorageManager {
    pub fn new() -> Self {
        MySQLStorageManager {
            values: read_mysql_json(),
        }
    }
}

impl MysqlStorageService for MySQLStorageManager {
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
        write_mysql_json(json);
    }

    fn add_username_password_model(&mut self, param: MysqlUserPassword) {
        let uid = Uuid::new_v4();

        let v = MySQLStorageEntity {
            r#type: USER_PASSWORD_MODEL,
            id: uid.to_string(),
            username: param.username,
            password: param.password,
            host: param.host,
            port: param.port,
            name: param.name,
        };
        self.values.push(v);
        self.write_config_json();
    }

    fn by_id(&self, id: &str) -> Option<&MySQLStorageEntity> {
        self.values.iter().find(|entity| entity.id == id)
    }
}

pub fn write_mysql_json(data: String) {
    if let Ok(home_directory) = get_home_directory().ok_or("无法获取 HOME 目录") {
        write_string_to_file(
            format!("{}/{}/{}", home_directory, HOME_NAME, MYSQL_CONFIG_JSON).as_str(),
            data.as_str(),
        )
        .expect("写入mysql-json失败");
    }
}

pub fn read_mysql_json() -> Vec<MySQLStorageEntity> {
    if let Ok(home_directory) = get_home_directory().ok_or("无法获取 HOME 目录") {
        let config_path = format!("{}/{}/{}", home_directory, HOME_NAME, MYSQL_CONFIG_JSON);
        if let Ok(file_contents) = read_file_content(config_path.as_str()) {
            if let Ok(parsed_data) = serde_json::from_str::<Vec<MySQLStorageEntity>>(&file_contents)
            {
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
