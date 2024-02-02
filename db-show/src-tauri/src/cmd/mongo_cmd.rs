use crate::cmd::cmd_entity::GetMongoInfoParam;
use crate::config::mongo_config::MongoUserPassword;
use crate::config::mongo_config_storage::{
    MongoStorageEntity, MongoStorageManager, MongoStorageService,
};
use crate::op::mongo_entity::MongoServerInfoCol;
use crate::op::mongo_op::MongoOperation;
use crate::resp::resp::Response;

#[tauri::command]
pub async fn mongo_info(param: GetMongoInfoParam) -> Result<Response<MongoServerInfoCol>, ()> {
    let manager = MongoStorageManager::new();

    let option = manager.by_id(param.id.as_str());

    let b = match option {
        None => Ok(Response::from_error("没有数据")),
        Some(&ref item) => {
            let mongo_user_password = MongoUserPassword {
                username: format!("{}", item.username),
                password: format!("{}", item.password),
                host: format!("{}", item.host),
                port: item.port,
                name: format!("{}", item.name),
            };
            let operation = MongoOperation::new(&mongo_user_password).await.unwrap();
            let result = operation.get_server_info().await;
            Ok(result)
        }
    };
    b
}

#[tauri::command]
pub async fn mongo_db_names(param: GetMongoInfoParam) -> Result<Response<Vec<String>>, ()> {
    let manager = MongoStorageManager::new();

    let option = manager.by_id(param.id.as_str());

    let b = match option {
        None => Ok(Response::from_error("没有数据")),
        Some(&ref item) => {
            let mongo_user_password = MongoUserPassword {
                username: format!("{}", item.username),
                password: format!("{}", item.password),
                host: format!("{}", item.host),
                port: item.port,
                name: format!("{}", item.name),
            };
            let operation = MongoOperation::new(&mongo_user_password).await.unwrap();
            let result = operation.db_names().await;
            Ok(result)
        }
    };
    b
}

#[tauri::command]
pub fn query_all_mongo() -> Response<Vec<MongoStorageEntity>> {
    let manager = MongoStorageManager::new();
    Response::new("a", Some(manager.values))
}

#[tauri::command]
pub fn add_mongo_config(param: MongoUserPassword) -> Response<bool> {
    let mut manager = MongoStorageManager::new();
    manager.add_username_password_model(param);
    Response::ok()
}
