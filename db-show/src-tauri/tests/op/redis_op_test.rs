#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use db_show::config::redis_config::RedisUserPassword;
    use db_show::op::redis_op::RedisOperation;

    #[test]
    pub fn test_server_info() {
        let operation = get_redis_op();
        let response = operation.get_server_info();
        dbg!(response);
    }

    #[test]
    pub fn c() {
        let operation = get_redis_op();
        let response = operation.get_db_size();
        dbg!(response);
    }

    #[test]
    pub fn test_get_keys_page() {
        let operation = get_redis_op();

        let db_index = 4;
        let page = 0;
        let page_size = 1;
        let result = operation.get_keys_page(db_index, page, page_size);
        dbg!(result);
        println!();
    }

    #[test]
    pub fn test_loop_get_keys_page() {
        let operation = get_redis_op();

        let db_index = 4;
        let mut page = 0;
        let page_size = 1;

        loop {
            // 调用 get_keys_page 方法获取一页的键和类型
            let result = operation.get_keys_page(db_index, page, page_size);
            dbg!(&result);
            // 打印当前页的键和类型
            match result.data {
                Some(scan_key_result) => {
                    for key in scan_key_result.keys {
                        println!(
                            "Key: {}, Type: {:?}, ttl {:?}",
                            key.key_name, key.key_type, key.ttl
                        );
                    }

                    println!("======");
                    // 如果还有下一页，更新页数，否则退出循环
                    if scan_key_result.new_cursor != 0 {
                        page = scan_key_result.new_cursor;
                    } else {
                        break;
                    }
                }
                None => {
                    // 打印错误信息并退出循环
                    println!("Error: {}", result.description);
                    break;
                }
            }
        }
    }

    #[test]
    pub fn test_get_string_data() {
        let operation = get_redis_op();

        let db_index = 1;
        let result = operation.get_string_data(db_index, "c".to_string());
        dbg!(result);
        println!();
    }

    #[test]
    pub fn test_get_list_data() {
        let operation = get_redis_op();

        let db_index = 1;
        let result = operation.get_list_data(db_index, "bac".to_string(), 0, 1);
        dbg!(result);
        println!();
    }

    #[test]
    pub fn test_get_set_data() {
        let operation = get_redis_op();

        let db_index = 1;
        let result = operation.get_set_data(db_index, "bb".to_string());

        dbg!(result);
        println!();
    }

    #[test]
    pub fn test_get_hash_data() {
        let operation = get_redis_op();

        let db_index = 1;
        let result = operation.get_hash_data(db_index, "hh".to_string());

        dbg!(result);
        println!();
    }

    #[test]
    pub fn test_get_zset_data() {
        let operation = get_redis_op();

        let db_index = 1;
        let result = operation.get_zset_data(db_index, "zset".to_string(), 0, 2);

        dbg!(result);
        println!();
    }

    #[test]
    pub fn test_set_string_data() {
        let operation = get_redis_op();

        let db_index = 1;
        let result = operation.set_string_data(db_index, "str".to_string(), "2".to_string());

        dbg!(result);
        println!();
    }

    #[test]
    pub fn test_set_list_data() {
        let operation = get_redis_op();

        let db_index = 1;
        let values = vec![
            "value1".to_string(),
            "value2".to_string(),
            "value3".to_string(),
        ];

        let result = operation.set_list_data(db_index, "list".to_string(), values);

        dbg!(result);
        println!();
    }

    #[test]
    pub fn test_set_set_data() {
        let operation = get_redis_op();

        let db_index = 1;
        let values = vec![
            "value1".to_string(),
            "value2".to_string(),
            "value3".to_string(),
        ];

        let result = operation.set_set_data(db_index, "set".to_string(), values);

        dbg!(result);
        println!();
    }

    #[test]
    pub fn test_set_hash_data() {
        let operation = get_redis_op();

        let db_index = 1;

        let mut fields_and_values = HashMap::new();
        fields_and_values.insert("field1".to_string(), "value1".to_string());
        fields_and_values.insert("field2".to_string(), "value2".to_string());

        // 调用 set_hash_data 函数进行设置
        let result = operation.set_hash_data(db_index, "hash".to_string(), fields_and_values);

        dbg!(result);
        println!();
    }

    #[test]
    pub fn test_set_zset_data() {
        let operation = get_redis_op();

        let db_index = 1;

        let mut members_scores = BTreeMap::new();
        members_scores.insert("member1".to_string(), 1.0);
        members_scores.insert("member2".to_string(), 2.0);
        members_scores.insert("member3".to_string(), 3.0);

        // 调用 set_hash_data 函数进行设置
        let result = operation.set_zset_data(db_index, "zset".to_string(), members_scores);

        dbg!(result);
        println!();
    }

    #[test]
    pub fn test_get_db_key_count() {
        let operation = get_redis_op();

        let db_index = 1;

        // 调用 set_hash_data 函数进行设置
        let result = operation.get_db_key_count(db_index);

        dbg!(result);
        println!();
    }

    #[test]
    pub fn test_change_set() {
        let operation = get_redis_op();

        let db_index = 1;

        // 调用 set_hash_data 函数进行设置
        let result = operation.change_set(
            db_index,
            "set".to_string(),
            "value3111".to_string(),
            "v2".to_string(),
        );

        dbg!(result);
        println!();
    }

    #[test]
    pub fn test_delete_redis_key() {
        let operation = get_redis_op();

        let db_index = 1;

        // 调用 set_hash_data 函数进行设置
        let result = operation.delete_redis_key(db_index, "del1".to_string());

        dbg!(result);
        println!();
    }

    #[test]
    pub fn test_set_redis_key_expire() {
        let operation = get_redis_op();

        let db_index = 1;

        // 调用 set_hash_data 函数进行设置
        let result = operation.set_redis_key_expire(db_index, "baca".to_string(), 100);

        dbg!(result);
        println!();
    }

    fn get_redis_op() -> RedisOperation {
        // let redis_config = RedisUserPassword {
        //     username: "root".to_string(),
        //     password: "7nd6dnH54vGcaXjUfH2G".to_string(),
        //     host: "116.62.61.104".to_string(),
        //     port: 8379,
        //     name: "your_database".to_string(),
        // };
        let redis_config = RedisUserPassword {
            username: "".to_string(),
            password: "".to_string(),
            host: "192.168.1.11".to_string(),
            port: 6379,
            name: "your_database".to_string(),
        };
        let redis_operation = RedisOperation::new(&redis_config).unwrap();
        redis_operation
    }
}
