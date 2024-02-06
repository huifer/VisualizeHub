use crate::cmd::cmd_entity::{CreateZookeeperDataParam, GetZookeeperChildrenParam, GetZookeeperDataParam, SetZookeeperDataParam};
use crate::config::zk_config::ZkUserPassword;
use crate::config::zk_config_storage::{ZkStorageEntity, ZkStorageManager, ZkStorageService};
use crate::op::zk_op::{ZookeeperData, ZookeeperOperation};
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


#[tauri::command]
pub async fn set_zookeeper_data(param: SetZookeeperDataParam) -> Result<Response<String>, ()> {
    let manager = ZkStorageManager::new();
    let option = manager.by_id(param.id.as_str());

    let b = match option {
        None => {
            Ok(Response::from_error("没有数据"))
        }
        Some(item) => {
            let operation = ZookeeperOperation::new(format!("{}", item.url));
            operation.set_zookeeper_data(param.path, param.string_data).await.expect("set zookeeper data error ");
            let response = Response::new("设置数据成功", Some("设置数据成功".to_string()));
            Ok(response)
        }
    };
    b
}

#[tauri::command]
pub async fn get_children_of_parent(param: GetZookeeperChildrenParam) -> Result<Response<Vec<String>>, ()> {
    let manager = ZkStorageManager::new();
    let option = manager.by_id(param.id.as_str());

    let b = match option {
        None => {
            Ok(Response::from_error("没有数据"))
        }
        Some(item) => {
            let operation = ZookeeperOperation::new(format!("{}", item.url));
            let option1 = operation.get_children_of_parent(param.path.as_str()).await.expect("get_children_of_parent error ");
            let response = Response::new("获取成功", option1);
            Ok(response)
        }
    };
    b
}

#[tauri::command]
pub async fn get_zookeeper_data(param: GetZookeeperDataParam) -> Result<Response<ZookeeperData>, ()> {
    let manager = ZkStorageManager::new();
    let option = manager.by_id(param.id.as_str());

    let b = match option {
        None => {
            Ok(Response::from_error("没有数据"))
        }
        Some(item) => {
            let operation = ZookeeperOperation::new(format!("{}", item.url));
            let option1 = operation.get_zookeeper_data(param.path.as_str()).await.expect("get_children_of_parent error ");
            let response = Response::new("获取成功", Some(option1));
            Ok(response)
        }
    };
    b
}


#[tauri::command]
pub async fn create_and_set_data(param: CreateZookeeperDataParam) -> Result<Response<String>, ()> {
    let manager = ZkStorageManager::new();
    let option = manager.by_id(param.id.as_str());

    let b = match option {
        None => {
            Ok(Response::from_error("没有数据"))
        }
        Some(item) => {
            let operation = ZookeeperOperation::new(format!("{}", item.url));
            let option1 = operation.create_and_set_data(param.path, param.string_data).await.expect("get_children_of_parent error ");
            let response = Response::new("获取成功", Some(option1));
            Ok(response)
        }
    };
    b
}