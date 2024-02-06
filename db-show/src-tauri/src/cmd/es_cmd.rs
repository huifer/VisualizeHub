use crate::config::es_config::ESUserPassword;
use crate::config::es_config_storage::{ESStorageEntity, ESStorageManager, ESStorageService};
use crate::resp::resp::Response;

#[tauri::command]
pub fn query_all_es() -> Response<Vec<ESStorageEntity>> {
    let manager = ESStorageManager::new();
    Response::new("a", Some(manager.values))
}

#[tauri::command]
pub fn add_es_config(param: ESUserPassword) -> Response<bool> {
    let mut manager = ESStorageManager::new();
    manager.add_username_password_model(param);
    Response::ok()
}
