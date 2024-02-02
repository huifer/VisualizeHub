#[cfg(test)]
mod tests {
    use db_show::config::mysql_config::MysqlUserPassword;
    use db_show::op::mysql_op::MysqlOperation;
    use db_show::resp::resp::{Response, SUCCESS};

    #[cfg(test)]
    mod tests {
        use crate::op::mysql_op_test::tests::get_mysql_operation;

        #[tokio::test]
        async fn test_mysql_operation() {
            let mysql_operation = get_mysql_operation().await;

            let response = mysql_operation.get_database_names().await;
            if let Some(databases) = response.data {
                for db_name in databases {
                    println!("db_name {}", db_name);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_database_info() {
        let mysql_operation = get_mysql_operation().await;

        // 执行 get_database_info 方法
        let result = mysql_operation.get_database_info().await;
        dbg!(&result);
        // 验证结果
        match result {
            Response { status_code, data, .. } => {
                assert_eq!(status_code, SUCCESS);

                if let Some(database_info) = data {
                    // 验证数据库版本和状态信息
                    assert!(!database_info.version.is_empty());
                    assert!(!database_info.status.is_empty());
                } else {
                    panic!("Expected Some(DatabaseInfo), but got None.");
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_supported_encodings() {
        let mysql_operation = get_mysql_operation().await;


        // 执行 get_supported_encodings 方法
        let result = mysql_operation.get_supported_encodings().await;

        // 输出结果
        dbg!(&result);

        // 验证结果
        match result {
            Response { status_code, data, .. } => {
                assert_eq!(status_code, SUCCESS);

                if let Some(encodings) = data {
                    // 输出支持的编码集合
                    dbg!(&encodings);

                    // 验证支持的编码集合不为空
                    assert!(!encodings.is_empty());
                } else {
                    panic!("Expected Some(Vec<String>), but got None.");
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_collation() {
        let mysql_operation = get_mysql_operation().await;


        // 执行 get_collation 方法
        let result = mysql_operation.get_collation(Some("armscii8")).await;

        // 输出结果
        dbg!(&result);

        // 验证结果
        match result {
            Response { status_code, data, .. } => {
                assert_eq!(status_code, SUCCESS);

                if let Some(collations) = data {
                    // 输出排序规则信息
                    dbg!(&collations);

                    // 验证排序规则信息不为空
                    assert!(!collations.is_empty());

                    // 验证排序规则信息的结构是否正确
                    for collation in collations {
                        assert!(!collation.collation.is_empty());
                        assert!(!collation.charset.is_empty());
                        assert_ne!(collation.id, 0);
                        // 根据实际情况添加其他验证
                    }
                } else {
                    panic!("Expected Some(Vec<CollationInfo>), but got None.");
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_table_create_statement() {
        // 获取 MysqlOperation 实例
        let mysql_operation = get_mysql_operation().await;

        // 指定要查询的数据库和表
        let database_name = "chunan";
        let table_name = "device_data";

        // 执行 get_table_create_statement 方法
        let result = mysql_operation.get_table_create_statement(database_name, table_name).await;

        // 输出结果
        dbg!(&result);

        // 验证结果
        match result {
            Response { status_code, data, .. } => {
                assert_eq!(status_code, SUCCESS);

                if let Some(create_statement) = data {
                    // 输出表的创建语句
                    dbg!(&create_statement);


                    // 根据实际情况添加其他验证
                } else {}
            }
        }
    }

    #[tokio::test]
    async fn test_get_table_columns_info() {
        let mysql_operation = get_mysql_operation().await;

        // 执行 get_table_columns_info 方法
        let result = mysql_operation.get_table_columns_info("chunan", "construction_data_files").await;

        // 输出结果
        dbg!(&result);

        // 验证结果
        match result {
            Response { status_code, data, .. } => {
                assert_eq!(status_code, SUCCESS);

                if let Some(columns_info) = data {
                    // 输出字段信息
                    dbg!(&columns_info);

                    // 验证字段信息不为空
                    assert!(!columns_info.is_empty());

                    // 验证字段信息的结构是否正确
                    for column_info in columns_info {
                        assert!(!column_info.field.is_empty());
                        assert!(!column_info.ty.is_empty());
                        // 根据实际情况添加其他验证
                    }
                } else {
                    panic!("Expected Some(Vec<ColumnInfo>), but got None.");
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_table_indexes_info() {
        let mysql_operation = get_mysql_operation().await;


        // 执行 get_table_indexes_info 方法
        let result = mysql_operation.get_table_indexes_info("chunan", "construction_data_files").await;

        // 输出结果
        dbg!(&result);

        // 验证结果
        match result {
            Response { status_code, data, .. } => {
                assert_eq!(status_code, SUCCESS);

                if let Some(indexes_info) = data {
                    // 输出索引信息
                    dbg!(&indexes_info);

                    // 验证索引信息不为空
                    assert!(!indexes_info.is_empty());

                    // 验证索引信息的结构是否正确
                    for index_info in indexes_info {
                        assert!(!index_info.index_name.is_empty());
                        assert!(!index_info.column_name.is_empty());
                        assert!(!index_info.index_type.is_empty());
                        // 根据实际情况添加其他验证
                    }
                } else {
                    panic!("Expected Some(Vec<IndexInfo>), but got None.");
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_table_foreign_keys_info() {
        let mysql_operation = get_mysql_operation().await;
        let result = mysql_operation
            .get_table_foreign_keys_info("chunan", "construction_data_files")
            .await;

        // 输出结果
        dbg!(&result);

        // 验证结果
    }

    #[tokio::test]
    async fn test_get_detailed_table_info() {
        let mysql_operation = get_mysql_operation().await;


        // 指定数据库名和表名
        let database_name = "chunan";
        let table_name = "construction_data_files";

        // 执行测试函数
        let result = mysql_operation.get_detailed_table_info(database_name, table_name).await;
        dbg!(&result);
    }


    async fn get_mysql_operation() -> MysqlOperation {
        let credentials = MysqlUserPassword {
            username: "1".to_string(),
            password: "1".to_string(),
            host: "1".to_string(),
            port: 1,
            name: "测试数据库".to_string(),
        };
        let mysql_operation = MysqlOperation::new(&credentials).await.unwrap();
        mysql_operation
    }
}