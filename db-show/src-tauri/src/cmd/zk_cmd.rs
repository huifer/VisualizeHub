use crate::config::zk_config::ZkUserPassword;
use crate::config::zk_config_storage::{ZkStorageEntity, ZkStorageManager, ZkStorageService};
use crate::resp::resp::Response;

#[tauri::command]
pub fn query_all_zookeeper() -> Response<Vec<ZkStorageEntity>> {
    let manager = ZkStorageManager::new();
    Response::new("success", Some(manager.values))
}


#[tauri::command]
pub fn add_zookeeper_config(param: ZkUserPassword) -> Response<bool> {
    let mut manager = ZkStorageManager::new();
    manager.add_username_password_model(param);
    Response::ok()
}
