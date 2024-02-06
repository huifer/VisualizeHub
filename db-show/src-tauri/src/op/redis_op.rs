use std::collections::{BTreeMap, HashMap, HashSet};

use redis::InfoDict;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::config::redis_config::RedisUserPassword;
use crate::resp::resp::Response;

pub struct RedisOperation {
    client: redis::Client,
}

impl RedisOperation {
    pub fn new(config: &RedisUserPassword) -> Result<Self, redis::RedisError> {
        let connection_str = if config.password.is_empty() {
            format!("redis://{}:{}/", config.host, config.port)
        } else {
            format!(
                "redis://{}:{}@{}:{}/",
                config.username, config.password, config.host, config.port
            )
        };

        let client = redis::Client::open(connection_str).unwrap();
        Ok(Self { client })
    }
    pub fn get_db_size(&self) -> Response<i32> {
        let result = self.client.get_connection();
        match result {
            Ok(mut con) => {
                // 使用 CONFIG get databases 命令
                let result: Result<(String, String), redis::RedisError> = redis::cmd("CONFIG")
                    .arg("get")
                    .arg("databases")
                    .query(&mut con);

                return match result {
                    Ok((_v1, v2)) => Response::new("获取服务端信息成功", Some(v2.parse().unwrap())),
                    Err(err) => Response::from_error(format!("Error: {:?}", err)),
                };
            }
            Err(e) => {
                // 构建一个包含错误信息的 Response 实例
                Response::from_error(format!("Redis 链接异常: {}", e))
            }
        }
    }
    // 获取 Redis 服务端信息
    pub fn get_server_info(&self) -> Response<RedisInfo> {
        let result = self.client.get_connection();
        match result {
            Ok(mut con) => {
                // 使用 INFO 命令获取字符串格式的服务端信息
                let info: String = redis::cmd("INFO")
                    .arg("Keyspace")
                    .query(&mut con)
                    .expect("Failed to execute INFO command");

                let option = crate::op::redis_op::KeyspaceInfo::from_string(info.as_str());

                // 使用 INFO 命令获取字典格式的服务端信息
                let info_dict: redis::InfoDict = redis::cmd("INFO")
                    .query(&mut con)
                    .expect("Failed to execute INFO command");

                // 将字典格式的服务端信息转换为你的 RedisInfo 结构体
                let mut redis_info: RedisInfo = info_dict.into();
                redis_info.keyspace = option.unwrap().dbs;
                // 构建一个成功的 Response 实例并包含 RedisInfo 数据
                Response::new("获取服务端信息成功", Some(redis_info))
            }
            Err(e) => {
                // 构建一个包含错误信息的 Response 实例
                Response::from_error(format!("Redis 链接异常: {}", e))
            }
        }
    }

    pub fn get_keys_page(
        &self,
        db_index: i32,
        page: usize,
        page_size: usize,
    ) -> Response<ScanKeyResult> {
        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                // 计算当前页的游标位置
                let cursor: usize = page * page_size;

                // 使用 SCAN 命令进行分页迭代
                let result: Result<(usize, Vec<String>), redis::RedisError> = redis::cmd("SCAN")
                    .arg(cursor)
                    .arg("MATCH")
                    .arg("*")
                    .arg("COUNT")
                    .arg(page_size as isize)
                    .query(&mut con);
                match result {
                    Ok((new_cursor, keys)) => {
                        // 构建 HashMap<String, KeyType>，其中键为键名，值为键的类型
                        let key_info_list: Vec<KeyInfo> = keys
                            .iter()
                            .map(|key| {
                                let key_type_str: String = redis::cmd("TYPE")
                                    .arg(key)
                                    .query(&mut con)
                                    .unwrap_or_else(|_| "unknown".to_string());

                                let key_type = KeyType::from_string(&key_type_str);
                                let ttl_result = self.get_ttl(key, db_index);
                                let ttl = match ttl_result {
                                    Ok(Some(value)) => value,
                                    Ok(None) => -1, // Key does not exist
                                    Err(_) => -2,   // Error in getting TTL
                                };

                                KeyInfo {
                                    key_name: key.clone(),
                                    key_type,
                                    ttl,
                                }
                            })
                            .collect();
                        // println!("new_cursor {}", new_cursor);
                        // 返回包装在 ScanResult 中的结果
                        Response::new(
                            "操作成功",
                            Some(ScanKeyResult {
                                new_cursor,
                                keys: key_info_list,
                            }),
                        )
                    }
                    Err(err) => Response::from_error(format!("Error: {:?}", err)),
                }
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }
    pub fn get_db_key_count(&self, db_index: i32) -> Response<usize> {
        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                // 使用 DBSIZE 命令获取键的数量
                let result: Result<usize, redis::RedisError> = redis::cmd("DBSIZE").query(&mut con);

                match result {
                    Ok(count) => Response::new("获取键数量成功", Some(count)),
                    Err(err) => Response::from_error(format!("Error: {:?}", err)),
                }
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    /// fixme:
    ///  1. 类型校验
    ///  2. 性能问题，这里都是直接获取所有，数据量大的话不合适

    pub fn get_string_data(&self, db_index: i32, key: String) -> Response<String> {
        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                match redis::cmd("GET").arg(&key).query(&mut con) {
                    Ok(value) => Response::new("获取数据成功", Some(value)),
                    Err(err) => Response::from_error(format!("Error: {:?}", err)),
                }
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn get_list_data(
        &self,
        db_index: i32,
        key: String,
        start: isize,
        stop: isize,
    ) -> Response<ListData> {
        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                // 获取 List 数据
                let values: Vec<String> = match redis::cmd("LRANGE")
                    .arg(&key)
                    .arg(start)
                    .arg(stop - 1)
                    .query(&mut con)
                {
                    Ok(values) => values,
                    Err(err) => {
                        return Response::from_error(format!("Error: {:?}", err));
                    }
                };

                // 获取 List 的总长度
                let total_length: usize = match redis::cmd("LLEN").arg(&key).query(&mut con) {
                    Ok(length) => length,
                    Err(err) => {
                        return Response::from_error(format!("Error: {:?}", err));
                    }
                };

                // 返回包装在 ListData 中的结果
                Response::new("获取数据成功", Some(ListData::new(values, total_length)))
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn get_set_data(&self, db_index: i32, key: String) -> Response<SetData> {
        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                // 获取 Set 数据
                let values: HashSet<String> = match redis::cmd("SMEMBERS").arg(&key).query(&mut con)
                {
                    Ok(values) => values,
                    Err(err) => {
                        return Response::from_error(format!("Error: {:?}", err));
                    }
                };

                // 获取 Set 的总长度
                let total_length: usize = match redis::cmd("SCARD").arg(&key).query(&mut con) {
                    Ok(length) => length,
                    Err(err) => {
                        return Response::from_error(format!("Error: {:?}", err));
                    }
                };

                // 返回包装在 SetData 中的结果
                Response::new("获取数据成功", Some(SetData::new(values, total_length)))
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn get_hash_data(&self, db_index: i32, key: String) -> Response<HashData> {
        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                let values: Vec<HashEnt> = match redis::cmd("HGETALL")
                    .arg(&key)
                    .query::<Vec<String>>(&mut con)
                {
                    Ok(values) => {
                        // Convert values to Vec<HashEnt>
                        values
                            .chunks(2)
                            .map(|chunk| HashEnt {
                                key: chunk[0].clone(),
                                val: chunk[1].clone(),
                            })
                            .collect()
                    }
                    Err(err) => {
                        return Response::from_error(format!("Error: {:?}", err));
                    }
                }; // 获取 Hash 的总长度
                let total_length: usize = match redis::cmd("HLEN").arg(&key).query(&mut con) {
                    Ok(length) => length,
                    Err(err) => {
                        return Response::from_error(format!("Error: {:?}", err));
                    }
                };

                // 返回包装在 HashData 中的结果
                Response::new("获取数据成功", Some(HashData::new(values, total_length)))
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn get_zset_data(
        &self,
        db_index: i32,
        key: String,
        start: isize,
        stop: isize,
    ) -> Response<ZSetData> {
        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                // 获取 ZSet 数据
                let values: Vec<ZSetEnt> = match redis::cmd("ZRANGE")
                    .arg(&key)
                    .arg(start)
                    .arg(stop - 1)
                    .arg("WITHSCORES")
                    .query::<Vec<String>>(&mut con)
                {
                    Ok(values) => {
                        // Convert values to Vec<ZSetEnt>
                        values
                            .chunks(2)
                            .map(|chunk| ZSetEnt {
                                member: chunk[0].clone(),
                                score: chunk[1].parse().unwrap(),
                            })
                            .collect()
                    }
                    Err(err) => {
                        return Response::from_error(format!("Error: {:?}", err));
                    }
                };

                // 获取 ZSet 的总长度
                let total_length: usize = match redis::cmd("ZCARD").arg(&key).query(&mut con) {
                    Ok(length) => length,
                    Err(err) => {
                        return Response::from_error(format!("Error: {:?}", err));
                    }
                };

                // 返回包装在 ZSetData 中的结果
                Response::new("获取数据成功", Some(ZSetData::new(values, total_length)))
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn set_string_data(&self, db_index: i32, key: String, value: String) -> Response<bool> {
        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");
                // 使用 SET 命令设置 String 数据
                let result: Result<(), redis::RedisError> =
                    redis::cmd("SET").arg(&key).arg(&value).query(&mut con);

                match result {
                    Ok(_) => Response::new("设置数据成功", Some(true)),
                    Err(err) => Response::from_error(format!("Error: {:?}", err)),
                }
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn set_list_data(
        &self,
        db_index: i32,
        key: String,
        values: Vec<String>,
    ) -> Response<usize> {
        if values.is_empty() {
            return Response::from_error("值列表不能为空");
        }

        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                // 使用 RPUSH 命令将值推入列表的右端
                let result: Result<usize, redis::RedisError> =
                    redis::cmd("RPUSH").arg(&key).arg(values).query(&mut con);

                match result {
                    Ok(length) => Response::new("设置数据成功", Some(length)),
                    Err(err) => Response::from_error(format!("Error: {:?}", err)),
                }
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn set_set_data(
        &self,
        db_index: i32,
        key: String,
        members: Vec<String>,
    ) -> Response<usize> {
        if members.is_empty() {
            return Response::from_error("成员列表不能为空");
        }

        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                // 使用 SADD 命令将成员添加到集合中
                let result: Result<usize, redis::RedisError> =
                    redis::cmd("SADD").arg(&key).arg(members).query(&mut con);

                match result {
                    Ok(count) => Response::new("设置数据成功", Some(count)),
                    Err(err) => Response::from_error(format!("Error: {:?}", err)),
                }
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn set_hash_data(
        &self,
        db_index: i32,
        key: String,
        field_values: HashMap<String, String>,
    ) -> Response<String> {
        if field_values.is_empty() {
            return Response::from_error("字段值映射不能为空");
        }

        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                // 将 HashMap 转换为 Vec<(String, String)>
                let field_values_vec: Vec<(&str, &str)> = field_values
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();

                // 使用 HMSET 命令将多个 field-value 对设置到哈希表中
                let result: Result<(), redis::RedisError> = redis::cmd("HMSET")
                    .arg(&key)
                    .arg(field_values_vec)
                    .query(&mut con);

                match result {
                    Ok(_) => Response::new("设置数据成功", Some("OK".to_string())),
                    Err(err) => Response::from_error(format!("Error: {:?}", err)),
                }
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }
    pub fn remove_hash_hk_data(&self, db_index: i32, key: String, hk: String) -> Response<bool> {
        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                // 从 Hash 中删除指定的字段
                let removed_count: i64 = redis::cmd("HDEL")
                    .arg(key.as_str())
                    .arg(hk.as_str())
                    .query(&mut con)
                    .expect("Failed to remove field from hash");

                // 根据删除结果判断是否成功
                if removed_count > 0 {
                    Response::new("成功删除字段", Some(true))
                } else {
                    Response::new("字段不存在或删除失败", Some(false))
                }
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn set_zset_data(
        &self,
        db_index: i32,
        key: String,
        members_scores: BTreeMap<String, f64>,
    ) -> Response<usize> {
        if members_scores.is_empty() {
            return Response::from_error("成员分数映射不能为空");
        }

        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                // 将 BTreeMap 转换为 Vec<(f64, &str)>
                let members_scores_vec: Vec<(f64, &str)> = members_scores
                    .iter()
                    .map(|(k, v)| (*v, k.as_str()))
                    .collect();

                // 使用 ZADD 命令将成员及其分数添加到有序集合中
                let result: Result<usize, redis::RedisError> = redis::cmd("ZADD")
                    .arg(&key)
                    .arg(members_scores_vec)
                    .query(&mut con);

                match result {
                    Ok(count) => Response::new("设置数据成功", Some(count)),
                    Err(err) => Response::from_error(format!("Error: {:?}", err)),
                }
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }
    pub fn remove_member_from_zset(
        &self,
        db_index: i32,
        key: String,
        member: String,
    ) -> Response<usize> {
        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                // 使用 ZREM 命令删除指定成员
                let result: Result<usize, redis::RedisError> =
                    redis::cmd("ZREM").arg(&key).arg(member).query(&mut con);

                match result {
                    Ok(count) => Response::new("成功删除成员", Some(count)),
                    Err(err) => Response::from_error(format!("Error: {:?}", err)),
                }
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn get_ttl(&self, key: &str, db_index: i32) -> Result<Option<i64>, String> {
        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .map_err(|e| format!("Failed to SELECT database: {:?}", e))?;

                // 获取键的过期时间
                let ttl: i64 = redis::cmd("TTL")
                    .arg(key)
                    .query(&mut con)
                    .map_err(|e| format!("Failed to get TTL for key '{}': {:?}", key, e))?;

                // 返回过期时间（-1 表示键不存在，-2 表示键存在但没有设置过期时间）
                Ok(Some(ttl))
            }
            Err(e) => Err(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn change_set(
        &self,
        db_index: i32,
        set_key: String,
        old_value: String,
        new_value: String,
    ) -> Response<bool> {
        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                // 从Set中移除旧值
                redis::cmd("SREM")
                    .arg(set_key.as_str())
                    .arg(old_value.as_str())
                    .execute(&mut con);

                // 将新值添加到Set中
                redis::cmd("SADD")
                    .arg(set_key.as_str())
                    .arg(new_value.as_str())
                    .execute(&mut con);

                Response::new("设置数据成功", Some(true))
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn remove_set_value(
        &self,
        db_index: i32,
        set_key: String,
        old_value: String,
    ) -> Response<bool> {
        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                // 从Set中移除旧值
                redis::cmd("SREM")
                    .arg(set_key.as_str())
                    .arg(old_value.as_str())
                    .execute(&mut con);

                Response::new("设置数据成功", Some(true))
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn change_list(
        &self,
        db_index: i32,
        list_key: String,
        old_value: String,
        new_value: String,
    ) -> Response<bool> {
        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                // 从List中移除旧值
                let _removed_count: i32 = redis::cmd("LREM")
                    .arg(list_key.as_str())
                    .arg(0) // Remove all occurrences of the value
                    .arg(old_value.as_str())
                    .query(&mut con)
                    .expect("Failed to remove old value from list");

                // 将新值添加到List的开头
                redis::cmd("LPUSH")
                    .arg(list_key.as_str())
                    .arg(new_value.as_str())
                    .execute(&mut con);

                Response::new("设置数据成功", Some(true))
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn remove_list_value(
        &self,
        db_index: i32,
        list_key: String,
        old_value: String,
    ) -> Response<bool> {
        let result = self.client.get_connection();

        match result {
            Ok(mut con) => {
                // 切换到指定的数据库
                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut con)
                    .expect("Failed to SELECT database");

                // 从List中移除旧值
                let _removed_count: i32 = redis::cmd("LREM")
                    .arg(list_key.as_str())
                    .arg(0) // Remove all occurrences of the value
                    .arg(old_value.as_str())
                    .query(&mut con)
                    .expect("Failed to remove old value from list");

                Response::new("设置数据成功", Some(true))
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn delete_redis_key(&self, db_index: i32, key: String) -> Response<bool> {
        match self.client.get_connection() {
            Ok(mut con) => {
                // 切换到指定的数据库
                match redis::cmd("SELECT").arg(db_index).query::<()>(&mut con) {
                    Ok(_) => {
                        // 使用 DEL 删除指定的 key
                        redis::cmd("DEL").arg(key.as_str()).execute(&mut con);

                        Response::new("删除 key 成功", Some(true))
                    }
                    Err(err) => {
                        Response::from_error(format!("Failed to SELECT database: {:?}", err))
                    }
                }
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn set_redis_key_expire(
        &self,
        db_index: i32,
        key: String,
        expiration_seconds: usize,
    ) -> Response<bool> {
        match self.client.get_connection() {
            Ok(mut con) => {
                // 切换到指定的数据库
                match redis::cmd("SELECT").arg(db_index).query::<()>(&mut con) {
                    Ok(_) => {
                        // 使用 EXPIRE 设置 key 的过期时间
                        redis::cmd("EXPIRE")
                            .arg(key.as_str())
                            .arg(expiration_seconds)
                            .execute(&mut con);
                        Response::new("设置 key 过期时间成功", Some(true))
                    }
                    Err(err) => {
                        Response::from_error(format!("Failed to SELECT database: {:?}", err))
                    }
                }
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }

    pub fn persist_redis_key(&self, db_index: i32, key: &str) -> Response<bool> {
        match self.client.get_connection() {
            Ok(mut con) => {
                // 切换到指定的数据库
                match redis::cmd("SELECT").arg(db_index).query::<()>(&mut con) {
                    Ok(_) => {
                        // 使用 PERSIST 移除 key 的过期时间
                        redis::cmd("PERSIST").arg(key).execute(&mut con);
                        Response::new("设置 key 不过期成功", Some(true))
                    }
                    Err(err) => {
                        Response::from_error(format!("Failed to SELECT database: {:?}", err))
                    }
                }
            }
            Err(e) => Response::from_error(format!("Redis 链接异常: {}", e)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ZSetData {
    pub values: Vec<ZSetEnt>,
    pub total_length: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ZSetEnt {
    pub member: String,
    pub score: f64,
}

impl ZSetData {
    pub fn new(values: Vec<ZSetEnt>, total_length: usize) -> Self {
        Self {
            values,
            total_length,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HashData {
    pub values: Vec<HashEnt>,
    pub total_length: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HashEnt {
    pub key: String,
    pub val: String,
}

impl HashData {
    pub fn new(values: Vec<HashEnt>, total_length: usize) -> Self {
        Self {
            values,
            total_length,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetData {
    pub values: HashSet<String>,
    pub total_length: usize,
}

impl SetData {
    pub fn new(values: HashSet<String>, total_length: usize) -> Self {
        Self {
            values,
            total_length,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListData {
    pub values: Vec<String>,
    pub total_length: usize,
}

impl ListData {
    pub fn new(values: Vec<String>, total_length: usize) -> Self {
        Self {
            values,
            total_length,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScanKeyResult {
    pub new_cursor: usize,
    pub keys: Vec<KeyInfo>, // 使用 Vec<KeyInfo> 替代 HashMap<String, KeyType>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyInfo {
    pub key_name: String,
    pub key_type: KeyType,
    pub ttl: i64,
}

impl ScanKeyResult {
    pub fn new(new_cursor: usize, keys: Vec<KeyInfo>) -> Self {
        Self { new_cursor, keys }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum KeyType {
    String,
    List,
    Set,
    ZSet,
    Hash,
    None,
    Unknown,
}

impl KeyType {
    // 从字符串转换为 KeyType
    fn from_string(type_str: &str) -> Self {
        match type_str.to_lowercase().as_str() {
            "string" => KeyType::String,
            "list" => KeyType::List,
            "set" => KeyType::Set,
            "zset" => KeyType::ZSet,
            "hash" => KeyType::Hash,
            "none" => KeyType::None,
            _ => KeyType::Unknown,
        }
    }
}

impl KeyspaceInfo {
    fn from_string(input: &str) -> Option<Self> {
        let re = Regex::new(r"(?P<db_name>db\d+):keys=(?P<keys>\d+),expires=(?P<expires>\d+),avg_ttl=(?P<avg_ttl>\d+)")
            .unwrap();

        let mut dbs = Vec::new();

        for (index, captures) in re.captures_iter(input).enumerate() {
            let db_info = DbInfo {
                index: format!("{}", index), // Use index as a string
                keys: captures["keys"].parse().unwrap(),
                expires: captures["expires"].parse().unwrap(),
                avg_ttl: captures["avg_ttl"].parse().unwrap(),
            };
            dbs.push(db_info);
        }

        if !dbs.is_empty() {
            Some(KeyspaceInfo { dbs })
        } else {
            None
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct DbInfo {
    index: String,
    // Add an index field
    keys: i32,
    expires: i32,
    avg_ttl: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct KeyspaceInfo {
    dbs: Vec<DbInfo>, // Change dbs to Vec<DbInfo>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisInfo {
    server_info: ServerInfo,
    clients_info: ClientsInfo,
    memory_info: MemoryInfo,
    stats_info: StatsInfo,
    persistence_info: PersistenceInfo,
    replication_info: ReplicationInfo,
    cpu_info: CpuInfo,
    keyspace: Vec<DbInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerInfo {
    redis_version: String,
    redis_git_sha1: String,
    redis_git_dirty: String,
    redis_build_id: String,
    redis_mode: String,
    os: String,
    arch_bits: String,
    multiplexing_api: String,
    gcc_version: String,
    process_id: String,
    run_id: String,
    tcp_port: String,
    uptime_in_seconds: String,
    uptime_in_days: String,
    hz: String,
    lru_clock: String,
    executable: String,
    config_file: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClientsInfo {
    connected_clients: String,
    client_longest_output_list: String,
    client_biggest_input_buf: String,
    blocked_clients: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MemoryInfo {
    used_memory: String,
    used_memory_human: String,
    used_memory_rss: String,
    used_memory_rss_human: String,
    used_memory_peak: String,
    used_memory_peak_human: String,
    total_system_memory: String,
    total_system_memory_human: String,
    used_memory_lua: String,
    used_memory_lua_human: String,
    maxmemory: String,
    maxmemory_human: String,
    maxmemory_policy: String,
    mem_fragmentation_ratio: String,
    mem_allocator: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct StatsInfo {
    total_connections_received: String,
    total_commands_processed: String,
    instantaneous_ops_per_sec: String,
    total_net_input_bytes: String,
    total_net_output_bytes: String,
    instantaneous_input_kbps: String,
    instantaneous_output_kbps: String,
    rejected_connections: String,
    sync_full: String,
    sync_partial_ok: String,
    sync_partial_err: String,
    expired_keys: String,
    evicted_keys: String,
    keyspace_hits: String,
    keyspace_misses: String,
    pubsub_channels: String,
    pubsub_patterns: String,
    latest_fork_usec: String,
    migrate_cached_sockets: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistenceInfo {
    loading: String,
    rdb_changes_since_last_save: String,
    rdb_bgsave_in_progress: String,
    rdb_last_save_time: String,
    rdb_last_bgsave_status: String,
    rdb_last_bgsave_time_sec: String,
    rdb_current_bgsave_time_sec: String,
    aof_enabled: String,
    aof_rewrite_in_progress: String,
    aof_rewrite_scheduled: String,
    aof_last_rewrite_time_sec: String,
    aof_current_rewrite_time_sec: String,
    aof_last_bgrewrite_status: String,
    aof_last_write_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReplicationInfo {
    role: String,
    connected_slaves: String,
    master_repl_offset: String,
    repl_backlog_active: String,
    repl_backlog_size: String,
    repl_backlog_first_byte_offset: String,
    repl_backlog_histlen: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CpuInfo {
    used_cpu_sys: String,
    used_cpu_user: String,
    used_cpu_sys_children: String,
    used_cpu_user_children: String,
}

impl From<redis::InfoDict> for RedisInfo {
    fn from(info_dict: InfoDict) -> Self {
        RedisInfo {
            server_info: ServerInfo {
                redis_version: info_dict
                    .get::<String>("redis_version")
                    .unwrap_or_default()
                    .to_string(),
                redis_git_sha1: info_dict
                    .get::<String>("redis_git_sha1")
                    .unwrap_or_default()
                    .to_string(),
                redis_git_dirty: info_dict
                    .get::<String>("redis_git_dirty")
                    .unwrap_or_default()
                    .to_string(),
                redis_build_id: info_dict
                    .get::<String>("redis_build_id")
                    .unwrap_or_default()
                    .to_string(),
                redis_mode: info_dict
                    .get::<String>("redis_mode")
                    .unwrap_or_default()
                    .to_string(),
                os: info_dict
                    .get::<String>("os")
                    .unwrap_or_default()
                    .to_string(),
                arch_bits: info_dict
                    .get::<String>("arch_bits")
                    .unwrap_or_default()
                    .to_string(),
                multiplexing_api: info_dict
                    .get::<String>("multiplexing_api")
                    .unwrap_or_default()
                    .to_string(),
                gcc_version: info_dict
                    .get::<String>("gcc_version")
                    .unwrap_or_default()
                    .to_string(),
                process_id: info_dict
                    .get::<String>("process_id")
                    .unwrap_or_default()
                    .to_string(),
                run_id: info_dict
                    .get::<String>("run_id")
                    .unwrap_or_default()
                    .to_string(),
                tcp_port: info_dict
                    .get::<String>("tcp_port")
                    .unwrap_or_default()
                    .to_string(),
                uptime_in_seconds: info_dict
                    .get::<String>("uptime_in_seconds")
                    .unwrap_or_default()
                    .to_string(),
                uptime_in_days: info_dict
                    .get::<String>("uptime_in_days")
                    .unwrap_or_default()
                    .to_string(),
                hz: info_dict
                    .get::<String>("hz")
                    .unwrap_or_default()
                    .to_string(),
                lru_clock: info_dict
                    .get::<String>("lru_clock")
                    .unwrap_or_default()
                    .to_string(),
                executable: info_dict
                    .get::<String>("executable")
                    .unwrap_or_default()
                    .to_string(),
                config_file: info_dict
                    .get::<String>("config_file")
                    .unwrap_or_default()
                    .to_string(),
            },
            clients_info: ClientsInfo {
                connected_clients: info_dict
                    .get::<String>("connected_clients")
                    .unwrap_or_default()
                    .to_string(),
                client_longest_output_list: info_dict
                    .get::<String>("client_longest_output_list")
                    .unwrap_or_default()
                    .to_string(),
                client_biggest_input_buf: info_dict
                    .get::<String>("client_biggest_input_buf")
                    .unwrap_or_default()
                    .to_string(),
                blocked_clients: info_dict
                    .get::<String>("blocked_clients")
                    .unwrap_or_default()
                    .to_string(),
            },
            memory_info: MemoryInfo {
                used_memory: info_dict
                    .get::<String>("used_memory")
                    .unwrap_or_default()
                    .to_string(),
                used_memory_human: info_dict
                    .get::<String>("used_memory_human")
                    .unwrap_or_default()
                    .to_string(),
                used_memory_rss: info_dict
                    .get::<String>("used_memory_rss")
                    .unwrap_or_default()
                    .to_string(),
                used_memory_rss_human: info_dict
                    .get::<String>("used_memory_rss_human")
                    .unwrap_or_default()
                    .to_string(),
                used_memory_peak: info_dict
                    .get::<String>("used_memory_peak")
                    .unwrap_or_default()
                    .to_string(),
                used_memory_peak_human: info_dict
                    .get::<String>("used_memory_peak_human")
                    .unwrap_or_default()
                    .to_string(),
                total_system_memory: info_dict
                    .get::<String>("total_system_memory")
                    .unwrap_or_default()
                    .to_string(),
                total_system_memory_human: info_dict
                    .get::<String>("total_system_memory_human")
                    .unwrap_or_default()
                    .to_string(),
                used_memory_lua: info_dict
                    .get::<String>("used_memory_lua")
                    .unwrap_or_default()
                    .to_string(),
                used_memory_lua_human: info_dict
                    .get::<String>("used_memory_lua_human")
                    .unwrap_or_default()
                    .to_string(),
                maxmemory: info_dict
                    .get::<String>("maxmemory")
                    .unwrap_or_default()
                    .to_string(),
                maxmemory_human: info_dict
                    .get::<String>("maxmemory_human")
                    .unwrap_or_default()
                    .to_string(),
                maxmemory_policy: info_dict
                    .get::<String>("maxmemory_policy")
                    .unwrap_or_default()
                    .to_string(),
                mem_fragmentation_ratio: info_dict
                    .get::<String>("mem_fragmentation_ratio")
                    .unwrap_or_default()
                    .to_string(),
                mem_allocator: info_dict
                    .get::<String>("mem_allocator")
                    .unwrap_or_default()
                    .to_string(),
            },
            stats_info: StatsInfo {
                total_connections_received: info_dict
                    .get::<String>("total_connections_received")
                    .unwrap_or_default()
                    .to_string(),
                total_commands_processed: info_dict
                    .get::<String>("total_commands_processed")
                    .unwrap_or_default()
                    .to_string(),
                instantaneous_ops_per_sec: info_dict
                    .get::<String>("instantaneous_ops_per_sec")
                    .unwrap_or_default()
                    .to_string(),
                total_net_input_bytes: info_dict
                    .get::<String>("total_net_input_bytes")
                    .unwrap_or_default()
                    .to_string(),
                total_net_output_bytes: info_dict
                    .get::<String>("total_net_output_bytes")
                    .unwrap_or_default()
                    .to_string(),
                instantaneous_input_kbps: info_dict
                    .get::<String>("instantaneous_input_kbps")
                    .unwrap_or_default()
                    .to_string(),
                instantaneous_output_kbps: info_dict
                    .get::<String>("instantaneous_output_kbps")
                    .unwrap_or_default()
                    .to_string(),
                rejected_connections: info_dict
                    .get::<String>("rejected_connections")
                    .unwrap_or_default()
                    .to_string(),
                sync_full: info_dict
                    .get::<String>("sync_full")
                    .unwrap_or_default()
                    .to_string(),
                sync_partial_ok: info_dict
                    .get::<String>("sync_partial_ok")
                    .unwrap_or_default()
                    .to_string(),
                sync_partial_err: info_dict
                    .get::<String>("sync_partial_err")
                    .unwrap_or_default()
                    .to_string(),
                expired_keys: info_dict
                    .get::<String>("expired_keys")
                    .unwrap_or_default()
                    .to_string(),
                evicted_keys: info_dict
                    .get::<String>("evicted_keys")
                    .unwrap_or_default()
                    .to_string(),
                keyspace_hits: info_dict
                    .get::<String>("keyspace_hits")
                    .unwrap_or_default()
                    .to_string(),
                keyspace_misses: info_dict
                    .get::<String>("keyspace_misses")
                    .unwrap_or_default()
                    .to_string(),
                pubsub_channels: info_dict
                    .get::<String>("pubsub_channels")
                    .unwrap_or_default()
                    .to_string(),
                pubsub_patterns: info_dict
                    .get::<String>("pubsub_patterns")
                    .unwrap_or_default()
                    .to_string(),
                latest_fork_usec: info_dict
                    .get::<String>("latest_fork_usec")
                    .unwrap_or_default()
                    .to_string(),
                migrate_cached_sockets: info_dict
                    .get::<String>("migrate_cached_sockets")
                    .unwrap_or_default()
                    .to_string(),
            },
            persistence_info: PersistenceInfo {
                loading: info_dict
                    .get::<String>("loading")
                    .unwrap_or_default()
                    .to_string(),
                rdb_changes_since_last_save: info_dict
                    .get::<String>("rdb_changes_since_last_save")
                    .unwrap_or_default()
                    .to_string(),
                rdb_bgsave_in_progress: info_dict
                    .get::<String>("rdb_bgsave_in_progress")
                    .unwrap_or_default()
                    .to_string(),
                rdb_last_save_time: info_dict
                    .get::<String>("rdb_last_save_time")
                    .unwrap_or_default()
                    .to_string(),
                rdb_last_bgsave_status: info_dict
                    .get::<String>("rdb_last_bgsave_status")
                    .unwrap_or_default()
                    .to_string(),
                rdb_last_bgsave_time_sec: info_dict
                    .get::<String>("rdb_last_bgsave_time_sec")
                    .unwrap_or_default()
                    .to_string(),
                rdb_current_bgsave_time_sec: info_dict
                    .get::<String>("rdb_current_bgsave_time_sec")
                    .unwrap_or_default()
                    .to_string(),
                aof_enabled: info_dict
                    .get::<String>("aof_enabled")
                    .unwrap_or_default()
                    .to_string(),
                aof_rewrite_in_progress: info_dict
                    .get::<String>("aof_rewrite_in_progress")
                    .unwrap_or_default()
                    .to_string(),
                aof_rewrite_scheduled: info_dict
                    .get::<String>("aof_rewrite_scheduled")
                    .unwrap_or_default()
                    .to_string(),
                aof_last_rewrite_time_sec: info_dict
                    .get::<String>("aof_last_rewrite_time_sec")
                    .unwrap_or_default()
                    .to_string(),
                aof_current_rewrite_time_sec: info_dict
                    .get::<String>("aof_current_rewrite_time_sec")
                    .unwrap_or_default()
                    .to_string(),
                aof_last_bgrewrite_status: info_dict
                    .get::<String>("aof_last_bgrewrite_status")
                    .unwrap_or_default()
                    .to_string(),
                aof_last_write_status: info_dict
                    .get::<String>("aof_last_write_status")
                    .unwrap_or_default()
                    .to_string(),
            },
            replication_info: ReplicationInfo {
                role: info_dict
                    .get::<String>("role")
                    .unwrap_or_default()
                    .to_string(),
                connected_slaves: info_dict
                    .get::<String>("connected_slaves")
                    .unwrap_or_default()
                    .to_string(),
                master_repl_offset: info_dict
                    .get::<String>("master_repl_offset")
                    .unwrap_or_default()
                    .to_string(),
                repl_backlog_active: info_dict
                    .get::<String>("repl_backlog_active")
                    .unwrap_or_default()
                    .to_string(),
                repl_backlog_size: info_dict
                    .get::<String>("repl_backlog_size")
                    .unwrap_or_default()
                    .to_string(),
                repl_backlog_first_byte_offset: info_dict
                    .get::<String>("repl_backlog_first_byte_offset")
                    .unwrap_or_default()
                    .to_string(),
                repl_backlog_histlen: info_dict
                    .get::<String>("repl_backlog_histlen")
                    .unwrap_or_default()
                    .to_string(),
            },
            cpu_info: CpuInfo {
                used_cpu_sys: info_dict
                    .get::<String>("used_cpu_sys")
                    .unwrap_or_default()
                    .to_string(),
                used_cpu_user: info_dict
                    .get::<String>("used_cpu_user")
                    .unwrap_or_default()
                    .to_string(),
                used_cpu_sys_children: info_dict
                    .get::<String>("used_cpu_sys_children")
                    .unwrap_or_default()
                    .to_string(),
                used_cpu_user_children: info_dict
                    .get::<String>("used_cpu_user_children")
                    .unwrap_or_default()
                    .to_string(),
            },
            keyspace: Default::default(),
        }
    }
}
