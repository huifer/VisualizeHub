use influxdb2::models::{Buckets, Organizations};

use crate::cmd::cmd_entity::{
    GetListBucketsInfluxdb, GetListFieldsInfluxdb, GetListMeasurementTagInfluxdb,
    GetListMeasurementsInfluxdb, GetListOrgInfluxdb,
};
use crate::config::influxdb_config::InfluxDBUserPassword;
use crate::config::influxdb_config_storage::{
    InfluxDBStorageEntity, InfluxDBStorageManager, InfluxDBStorageService,
};
use crate::op::influxdb2_op::Influxdb2Operation;
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

#[tauri::command]
pub async fn get_list_buckets(param: GetListBucketsInfluxdb) -> Result<Response<Buckets>, ()> {
    let manager = InfluxDBStorageManager::new();
    let option = manager.by_id(param.id.as_str());

    let b = match option {
        None => Ok(Response::from_error("没有数据")),
        Some(item) => {
            if "2".eq(item.version.as_str()) {
                let v = InfluxDBUserPassword {
                    url: format!("{}", item.url),
                    name: format!("{}", item.name),
                    auth_token: format!("{}", item.auth_token),
                    org: format!("{}", item.org),
                    version: format!("{}", item.version),
                };
                let operation = Influxdb2Operation::new(v);

                let buckets = operation
                    .get_list_buckets(param.limit, param.offset)
                    .await
                    .expect("set influxdb get_list_buckets error ");
                let response = Response::new("设置数据成功", Some(buckets));
                Ok(response)
            } else {
                Err(())
            }
        }
    };
    b
}

#[tauri::command]
pub async fn get_list_organizations(
    param: GetListOrgInfluxdb,
) -> Result<Response<Organizations>, ()> {
    let manager = InfluxDBStorageManager::new();
    let option = manager.by_id(param.id.as_str());

    let b = match option {
        None => Ok(Response::from_error("没有数据")),
        Some(item) => {
            if "2".eq(item.version.as_str()) {
                let v = InfluxDBUserPassword {
                    url: format!("{}", item.url),
                    name: format!("{}", item.name),
                    auth_token: format!("{}", item.auth_token),
                    org: format!("{}", item.org),
                    version: format!("{}", item.version),
                };
                let operation = Influxdb2Operation::new(v);

                let buckets = operation
                    .get_list_organizations(param.limit, param.offset)
                    .await
                    .expect("set influxdb get_list_buckets error ");
                let response = Response::new("设置数据成功", Some(buckets));
                Ok(response)
            } else {
                Err(())
            }
        }
    };
    b
}

#[tauri::command]
pub async fn get_list_measurements(
    param: GetListMeasurementsInfluxdb,
) -> Result<Response<Vec<String>>, ()> {
    let manager = InfluxDBStorageManager::new();
    let option = manager.by_id(param.id.as_str());

    let b = match option {
        None => Ok(Response::from_error("没有数据")),
        Some(item) => {
            if "2".eq(item.version.as_str()) {
                let v = InfluxDBUserPassword {
                    url: format!("{}", item.url),
                    name: format!("{}", item.name),
                    auth_token: format!("{}", item.auth_token),
                    org: format!("{}", item.org),
                    version: format!("{}", item.version),
                };
                let operation = Influxdb2Operation::new(v);

                let buckets = operation
                    .get_list_measurements(param.bucket)
                    .await
                    .expect("set influxdb get_list_buckets error ");
                let response = Response::new("设置数据成功", Some(buckets));
                Ok(response)
            } else {
                Err(())
            }
        }
    };
    b
}

#[tauri::command]
pub async fn get_list_fields(param: GetListFieldsInfluxdb) -> Result<Response<Vec<String>>, ()> {
    let manager = InfluxDBStorageManager::new();
    let option = manager.by_id(param.id.as_str());

    let b = match option {
        None => Ok(Response::from_error("没有数据")),
        Some(item) => {
            if "2".eq(item.version.as_str()) {
                let v = InfluxDBUserPassword {
                    url: format!("{}", item.url),
                    name: format!("{}", item.name),
                    auth_token: format!("{}", item.auth_token),
                    org: format!("{}", item.org),
                    version: format!("{}", item.version),
                };
                let operation = Influxdb2Operation::new(v);

                let buckets = operation
                    .get_list_measurements(param.bucket)
                    .await
                    .expect("set influxdb get_list_buckets error ");
                let response = Response::new("设置数据成功", Some(buckets));
                Ok(response)
            } else {
                Err(())
            }
        }
    };
    b
}

#[tauri::command]
pub async fn get_list_measurement_tag_keys(
    param: GetListMeasurementTagInfluxdb,
) -> Result<Response<Vec<String>>, ()> {
    let manager = InfluxDBStorageManager::new();
    let option = manager.by_id(param.id.as_str());

    let b = match option {
        None => Ok(Response::from_error("没有数据")),
        Some(item) => {
            if "2".eq(item.version.as_str()) {
                let v = InfluxDBUserPassword {
                    url: format!("{}", item.url),
                    name: format!("{}", item.name),
                    auth_token: format!("{}", item.auth_token),
                    org: format!("{}", item.org),
                    version: format!("{}", item.version),
                };
                let operation = Influxdb2Operation::new(v);

                let buckets = operation
                    .get_list_measurement_tag_keys(param.bucket, param.measurement)
                    .await
                    .expect("set influxdb get_list_buckets error ");
                let response = Response::new("设置数据成功", Some(buckets));
                Ok(response)
            } else {
                Err(())
            }
        }
    };
    b
}
