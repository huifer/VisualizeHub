use crate::config::mqtt_config::MqttUserPassword;
use crate::config::mqtt_config_storage::{
    MqttStorageEntity, MqttStorageManager, MqttStorageService,
};
use crate::resp::resp::Response;

#[tauri::command]
pub fn query_all_mqtt() -> Response<Vec<MqttStorageEntity>> {
    let manager = MqttStorageManager::new();
    Response::new("a", Some(manager.values))
}

#[tauri::command]
pub fn add_mqtt_config(param: MqttUserPassword) -> Response<bool> {
    let mut manager = MqttStorageManager::new();
    manager.add_username_password_model(param);
    Response::ok()
}
