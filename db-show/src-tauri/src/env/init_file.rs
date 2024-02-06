use std::fs::{File, OpenOptions};
use std::io::Write;
use std::{env, fs, io};

pub static HOME_NAME: &str = "db-show";
pub static MYSQL_CONFIG_JSON: &str = "mysql.json";
pub static REDIS_CONFIG_JSON: &str = "redis.json";
pub static MONGO_CONFIG_JSON: &str = "mongo.json";
pub static ES_CONFIG_JSON: &str = "es.json";
pub static ZK_CONFIG_JSON: &str = "zk.json";
pub static INFLUXDB_CONFIG_JSON: &str = "influxdb.json";
pub static MQTT_CONFIG_JSON: &str = "mqtt.json";

pub fn init_home() {
    if let Ok(home_directory) = get_home_directory().ok_or("无法获取 HOME 目录") {
        let directory_path = format!("{}/{}", home_directory, HOME_NAME);
        if let Err(err) = create_directory_if_not_exists(directory_path.as_str()) {
            eprintln!("Error: {}", err);
        } else {
        }
        create_file_if_not_exists(
            format!("{}/{}/{}", home_directory, HOME_NAME, MYSQL_CONFIG_JSON).as_str(),
            "",
        )
        .expect("创建 mysql 配置失败");
        create_file_if_not_exists(
            format!("{}/{}/{}", home_directory, HOME_NAME, REDIS_CONFIG_JSON).as_str(),
            "",
        )
        .expect("创建 redis 配置失败");
        create_file_if_not_exists(
            format!("{}/{}/{}", home_directory, HOME_NAME, MONGO_CONFIG_JSON).as_str(),
            "",
        )
        .expect("创建 mongo 配置失败");
        create_file_if_not_exists(
            format!("{}/{}/{}", home_directory, HOME_NAME, ES_CONFIG_JSON).as_str(),
            "",
        )
        .expect("创建 es 配置失败");
        create_file_if_not_exists(
            format!("{}/{}/{}", home_directory, HOME_NAME, MQTT_CONFIG_JSON).as_str(),
            "",
        )
        .expect("创建 es 配置失败");
        create_file_if_not_exists(
            format!("{}/{}/{}", home_directory, HOME_NAME, ZK_CONFIG_JSON).as_str(),
            "",
        )
        .expect("创建 zookeeper 配置失败");
        create_file_if_not_exists(
            format!("{}/{}/{}", home_directory, HOME_NAME, INFLUXDB_CONFIG_JSON).as_str(),
            "",
        )
        .expect("创建 influxdb 配置失败");
    }
}

/// 获取系统用户目录
pub fn get_home_directory() -> Option<String> {
    if cfg!(windows) {
        env::var_os("USERPROFILE").map(|path| path.to_string_lossy().into_owned())
    } else {
        env::var_os("HOME").map(|path| path.to_string_lossy().into_owned())
    }
}

pub fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    // 尝试打开文件
    let file_contents = fs::read_to_string(file_path)?;

    // 返回文件内容
    Ok(file_contents)
}

fn create_directory_if_not_exists(dir_path: &str) -> Result<(), std::io::Error> {
    if !fs::metadata(dir_path).is_ok() {
        // 如果文件夹不存在，创建它
        fs::create_dir(dir_path)?;
        println!("文件夹 '{}' 不存在，已创建。", dir_path);
    } else {
    }
    Ok(())
}

fn create_file_if_not_exists(file_path: &str, content: &str) -> Result<(), io::Error> {
    // 检查文件是否存在
    if fs::metadata(file_path).is_ok() {
        return Ok(());
    }

    // 如果文件不存在，创建文件并写入内容
    let mut file = File::create(file_path)?;

    // 写入内容到文件
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn write_string_to_file(file_path: &str, content: &str) -> Result<(), io::Error> {
    // 以写入模式打开文件
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    // 写入字符串到文件
    write!(file, "{}", content)?;

    Ok(())
}
