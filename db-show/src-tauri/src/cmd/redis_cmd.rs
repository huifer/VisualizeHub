use crate::cmd::cmd_entity::{
    ChangeRedisListValueParam, ChangeRedisSetValueParam, ChangeRedisZSetValueParam,
    GetRedisInfoParam, GetRedisKeysParam, GetRedisValueParam, SetRedisHashParam,
    SetRedisValueParam,
};
use crate::config::redis_config::RedisUserPassword;
use crate::config::redis_config_storage::{
    RedisStorageEntity, RedisStorageManager, RedisStorageService,
};
use crate::op::redis_op::{
    HashData, ListData, RedisInfo, RedisOperation, ScanKeyResult, SetData, ZSetData,
};
use crate::resp::resp::Response;

#[tauri::command]
pub fn query_all_redis() -> Response<Vec<RedisStorageEntity>> {
    let manager = RedisStorageManager::new();
    Response::new("success", Some(manager.values))
}

#[tauri::command]
pub fn add_redis_config(param: RedisUserPassword) -> Response<bool> {
    let mut manager = RedisStorageManager::new();
    manager.add_username_password_model(param);
    Response::ok()
}

#[tauri::command]
pub fn redis_info(param: GetRedisInfoParam) -> Response<RedisInfo> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let redis_operation = RedisOperation::new(&redis_config).unwrap();
            redis_operation.get_server_info()
        }
    }
}

#[tauri::command]
pub fn redis_db_count(param: GetRedisInfoParam) -> Response<i32> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let redis_operation = RedisOperation::new(&redis_config).unwrap();
            redis_operation.get_db_size()
        }
    }
}

#[tauri::command]
pub fn redis_keys_page(param: GetRedisKeysParam) -> Response<ScanKeyResult> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();
            let result = operation.get_keys_page(param.db_index, param.page, param.page_size);

            return result;
        }
    }
}

#[tauri::command]
pub fn redis_get_string_data(param: GetRedisValueParam) -> Response<String> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();
            let result = operation.get_string_data(param.db_index, param.key_name.to_string());

            return result;
        }
    }
}

#[tauri::command]
pub fn redis_set_string_data(param: SetRedisValueParam) -> Response<bool> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();
            let result =
                operation.set_string_data(param.db_index, param.key_name.to_string(), param.value);

            return result;
        }
    }
}

#[tauri::command]
pub fn redis_get_list_data(param: GetRedisValueParam) -> Response<ListData> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();
            let result = operation.get_list_data(param.db_index, param.key_name.to_string(), 0, 0);

            return result;
        }
    }
}

#[tauri::command]
pub fn redis_get_set_data(param: GetRedisValueParam) -> Response<SetData> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();
            let result = operation.get_set_data(param.db_index, param.key_name.to_string());

            return result;
        }
    }
}

#[tauri::command]
pub fn redis_get_hash_data(param: GetRedisValueParam) -> Response<HashData> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();
            let result = operation.get_hash_data(param.db_index, param.key_name.to_string());

            return result;
        }
    }
}

#[tauri::command]
pub fn redis_change_hash(param: SetRedisHashParam) -> Response<String> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();

            for x in param.old_field_values {
                operation.remove_hash_hk_data(param.db_index, param.key_name.to_string(), x.0);
            }

            let result = operation.set_hash_data(
                param.db_index,
                param.key_name.to_string(),
                param.new_field_values,
            );

            return result;
        }
    }
}
#[tauri::command]
pub fn redis_remove_hash_member(param: SetRedisHashParam) -> Response<bool> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();

            for x in param.old_field_values {
                operation.remove_hash_hk_data(param.db_index, param.key_name.to_string(), x.0);
            }

            return Response::new("ok", Some(true));
        }
    }
}

#[tauri::command]
pub fn redis_get_zset_data(param: GetRedisValueParam) -> Response<ZSetData> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();
            let result = operation.get_zset_data(param.db_index, param.key_name.to_string(), 0, 0);

            return result;
        }
    }
}

#[tauri::command]
pub fn redis_delete_redis_key(param: GetRedisValueParam) -> Response<bool> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();
            let result = operation.delete_redis_key(param.db_index, param.key_name.to_string());

            return result;
        }
    }
}

#[tauri::command]
pub fn redis_set_redis_key_expire(param: GetRedisValueParam) -> Response<bool> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();
            if param.expiration_seconds > 0 {
                return operation.set_redis_key_expire(
                    param.db_index,
                    param.key_name,
                    param.expiration_seconds as usize,
                );
            } else {
                return operation.persist_redis_key(param.db_index, param.key_name.as_str());
            }
        }
    }
}

#[tauri::command]
pub fn redis_change_set(param: ChangeRedisSetValueParam) -> Response<bool> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();
            operation.change_set(param.db_index, param.key_name, param.old, param.new_val)
        }
    }
}

#[tauri::command]
pub fn redis_add_set(param: ChangeRedisSetValueParam) -> Response<usize> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();
            let alues = vec![param.new_val];
            operation.set_set_data(param.db_index, param.key_name, alues)
        }
    }
}

#[tauri::command]
pub fn redis_remove_set_member(param: ChangeRedisSetValueParam) -> Response<bool> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();
            operation.remove_set_value(param.db_index, param.key_name, param.old)
        }
    }
}

#[tauri::command]
pub fn redis_change_list(param: ChangeRedisListValueParam) -> Response<bool> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();
            operation.change_list(param.db_index, param.key_name, param.old, param.new_val)
        }
    }
}

#[tauri::command]
pub fn redis_remove_list_member(param: ChangeRedisListValueParam) -> Response<bool> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();
            operation.remove_list_value(param.db_index, param.key_name, param.old)
        }
    }
}

#[tauri::command]
pub fn redis_change_zset(param: ChangeRedisZSetValueParam) -> Response<usize> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());
    dbg!(&param);
    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();
            operation.set_zset_data(param.db_index, param.key_name, param.members_scores)
        }
    }
}
#[tauri::command]
pub fn redis_remove_zset_member(param: ChangeRedisZSetValueParam) -> Response<bool> {
    let manager = RedisStorageManager::new();
    let option = manager.by_id(param.db_config_id.as_str());
    dbg!(&param);

    match option {
        None => Response::from_error("没有数据"),
        Some(&ref entity) => {
            let redis_config = RedisUserPassword {
                username: format!("{}", entity.username),
                password: format!("{}", entity.password),
                host: format!("{}", entity.host),
                port: entity.port,
                name: format!("{}", entity.name),
            };
            let operation = RedisOperation::new(&redis_config).unwrap();
            for x in param.members_scores {
                operation.remove_member_from_zset(param.db_index, param.key_name.to_string(), x.0);
            }
            Response::new("ok", Some(true))
        }
    }
}
