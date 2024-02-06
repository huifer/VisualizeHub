use crate::config::influxdb_config::InfluxDBUserPassword;
use crate::config::influxdb_config_storage::{InfluxDBStorageEntity, InfluxDBStorageManager, InfluxDBStorageService};
use crate::resp::resp::Response;

#[tauri::command]
pub fn query_all_influxdb() -> Response<Vec<InfluxDBStorageEntity>> {
    let manager = InfluxDBStorageManager::new();
    Response::new("success", Some(manager.values))
}


#[tauri::command]
pub fn add_influxdb_config(param: InfluxDBUserPassword) -> Response<bool> {
    let mut manager = InfluxDBStorageManager::new();
    manager.add_username_password_model(param);
    Response::ok()
}
