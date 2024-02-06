// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::cmd::influxdb_cmd::*;
use crate::cmd::mongo_cmd::{add_mongo_config, mongo_db_names, mongo_info, query_all_mongo};
use crate::cmd::mysql_cmd::{
    add_mysql_config, get_db_names, get_db_status, get_tables_names, query_all_mysql,
    show_table_create_sql,
};
use crate::cmd::redis_cmd::{add_redis_config, query_all_redis, redis_add_set,
                            redis_change_hash,
                            redis_change_list,
                            redis_change_set,
                            redis_change_zset,
                            redis_db_count, redis_delete_redis_key, redis_get_hash_data,
                            redis_get_list_data,
                            redis_get_set_data,
                            redis_get_string_data,
                            redis_get_zset_data,
                            redis_info,
                            redis_keys_page,
                            redis_remove_hash_member,
                            redis_remove_list_member, redis_remove_set_member, redis_remove_zset_member,
                            redis_set_redis_key_expire,
                            redis_set_string_data, };
use crate::cmd::zk_cmd::*;
use crate::env::init_file::init_home;

pub mod cmd;
pub mod config;
pub mod env;
pub mod op;
pub mod resp;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    init_home();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            query_all_mysql,
            add_mysql_config,
            get_db_names,
            get_tables_names,
            get_db_status,
            show_table_create_sql,
            add_redis_config,
            query_all_redis,
            redis_info,
            mongo_info,
            query_all_mongo,
            add_mongo_config,
            mongo_db_names,
            redis_db_count,
            redis_keys_page,
            redis_get_string_data,
redis_get_list_data,
redis_get_set_data,
redis_get_hash_data,
redis_get_zset_data,
            redis_delete_redis_key,
redis_set_redis_key_expire,
            redis_change_set,
            redis_remove_set_member,
            redis_add_set,
            redis_change_list,
redis_remove_list_member,
            redis_set_string_data,
            redis_change_hash,
            redis_remove_hash_member,
            redis_remove_zset_member,
            redis_change_zset,
            query_all_influxdb,
add_influxdb_config,
            query_all_zookeeper,
add_zookeeper_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
