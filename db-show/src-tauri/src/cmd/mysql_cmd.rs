use crate::cmd::cmd_entity::{GetMysqlTablesCreatedParam, GetMysqlTablesParam};
use crate::config::mysql_config::MysqlUserPassword;
use crate::config::mysql_config_storage::{
    MySQLStorageEntity, MySQLStorageManager, MysqlStorageService,
};
use crate::op::mysql_op::DatabaseInfo;
use crate::resp::resp::Response;

#[tauri::command]
pub fn query_all_mysql() -> Response<Vec<MySQLStorageEntity>> {
    let manager = MySQLStorageManager::new();
    Response::new("success", Some(manager.values))
}

#[tauri::command]
pub fn add_mysql_config(param: MysqlUserPassword) -> Response<bool> {
    let mut manager = MySQLStorageManager::new();
    manager.add_username_password_model(param);
    Response::ok()
}

#[tauri::command]
pub async fn get_db_names(id: &str) -> Result<Response<Vec<String>>, ()> {
    let manager = MySQLStorageManager::new();
    let option = manager.by_id(id);

    let b = match option {
        None => Ok(Response::from_error("没有数据")),
        Some(&ref item) => {
            let param = MysqlUserPassword {
                username: format!("{}", item.username),
                password: format!("{}", item.password),
                host: format!("{}", item.host),
                port: item.port,
                name: format!("{}", item.name),
            };

            let operation = crate::op::mysql_op::MysqlOperation::new(&param)
                .await
                .unwrap();
            let response = operation.get_database_names().await;
            Ok(response)
        }
    };
    b
}

#[tauri::command]
pub async fn get_tables_names(param: GetMysqlTablesParam) -> Result<Response<Vec<String>>, ()> {
    let manager = MySQLStorageManager::new();
    let option = manager.by_id(param.id.as_str());

    let b = match option {
        None => Ok(Response::from_error("没有数据")),
        Some(&ref item) => {
            let mup = MysqlUserPassword {
                username: format!("{}", item.username),
                password: format!("{}", item.password),
                host: format!("{}", item.host),
                port: item.port,
                name: format!("{}", item.name),
            };

            let operation = crate::op::mysql_op::MysqlOperation::new(&mup)
                .await
                .unwrap();
            let response = operation
                .get_table_names_for_database(param.db_name.as_str())
                .await;
            Ok(response)
        }
    };
    b
}

#[tauri::command]
pub async fn get_db_status(id: &str) -> Result<Response<DatabaseInfo>, ()> {
    let manager = MySQLStorageManager::new();
    let option = manager.by_id(id);

    let b = match option {
        None => Ok(Response::from_error("没有数据")),
        Some(&ref item) => {
            let mup = MysqlUserPassword {
                username: format!("{}", item.username),
                password: format!("{}", item.password),
                host: format!("{}", item.host),
                port: item.port,
                name: format!("{}", item.name),
            };

            let operation = crate::op::mysql_op::MysqlOperation::new(&mup)
                .await
                .unwrap();
            let response = operation.get_database_info().await;
            Ok(response)
        }
    };
    b
}

#[tauri::command]
pub async fn show_table_create_sql(
    param: GetMysqlTablesCreatedParam,
) -> Result<Response<String>, ()> {
    let manager = MySQLStorageManager::new();
    let option = manager.by_id(param.id.as_str());

    let b = match option {
        None => Ok(Response::from_error("没有数据")),
        Some(&ref item) => {
            let mup = MysqlUserPassword {
                username: format!("{}", item.username),
                password: format!("{}", item.password),
                host: format!("{}", item.host),
                port: item.port,
                name: format!("{}", item.name),
            };

            let operation = crate::op::mysql_op::MysqlOperation::new(&mup)
                .await
                .unwrap();
            let response = operation
                .get_table_create_statement(param.db_name.as_str(), param.table_name.as_str())
                .await;
            Ok(response)
        }
    };
    b
}
