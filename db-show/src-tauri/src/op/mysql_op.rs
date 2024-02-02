use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlConnectOptions;
use sqlx_core::row::Row;

use crate::config::mysql_config::MysqlUserPassword;
use crate::resp::resp::Response;

pub struct MysqlOperation {
    pool: Pool<MySql>,
}


impl MysqlOperation {
    pub async fn new(credentials: &MysqlUserPassword) -> Result<Self, sqlx::Error> {
        let options = MySqlConnectOptions::new().username(&credentials.username).password(&credentials.password).host(&credentials.host).port(credentials.port);

        let pool = Pool::connect_with(options).await?;

        Ok(Self { pool })
    }

    pub async fn get_database_names(&self) -> Response<Vec<String>> {
        let query = "SHOW DATABASES";
        let result = sqlx::query(query).fetch_all(&self.pool).await;

        match result {
            Ok(rows) => {
                let databases: Vec<String> = rows.iter().map(|row| row.get(0)).collect();
                Response::new("Success", Some(databases))
            }
            Err(err) => Response::from_error(format!("Error: {}", err)),
        }
    }

    pub async fn get_table_names_for_database(&self, database_name: &str) -> Response<Vec<String>> {
        // Use a parameterized query to avoid SQL injection
        let query = format!("SHOW TABLES FROM {}", database_name);
        let result = sqlx::query(&query).fetch_all(&self.pool).await;

        match result {
            Ok(rows) => {
                let tables: Vec<String> = rows.iter().map(|row| row.get(0)).collect();
                Response::new("Success", Some(tables))
            }
            Err(err) => Response::from_error(format!("Error: {}", err)),
        }
    }


    pub async fn get_database_info(&self) -> Response<DatabaseInfo> {
        let version_query = "SELECT VERSION()";
        let status_query = "SHOW STATUS";

        let version_result = sqlx::query(version_query).fetch_one(&self.pool).await;
        let status_result = sqlx::query(status_query).fetch_all(&self.pool).await;

        match (version_result, status_result) {
            (Ok(version_row), Ok(status_rows)) => {
                let version: String = version_row.get(0);
                let status_info: Vec<StatusInfo> = status_rows.iter().map(|row| StatusInfo {
                    name: row.get(0),
                    value: row.get(1),
                }).collect();

                let database_info = DatabaseInfo {
                    version,
                    status: status_info,
                };

                Response::new("Success", Some(database_info))
            }
            (Err(err), _) => Response::from_error(format!("Error fetching version: {}", err)),
            (_, Err(err)) => Response::from_error(format!("Error fetching status: {}", err)),
        }
    }
    pub async fn get_supported_encodings(&self) -> Response<Vec<String>> {
        let query = "SHOW CHARACTER SET";
        match sqlx::query(query).fetch_all(&self.pool).await {
            Ok(rows) => {
                let encodings: Vec<String> = rows.iter().map(|row| row.get(0)).collect();
                Response::new("Success", Some(encodings))
            }
            Err(err) => Response::from_error(format!("Error: {}", err)),
        }
    }

    pub async fn get_collation(&self, charset: Option<&str>) -> Response<Vec<CollationInfo>> {
        let mut query = "SHOW COLLATION".to_string();

        // 如果提供了字符集参数，添加条件到查询中
        if let Some(_charset) = charset {
            query.push_str(" WHERE CHARSET = ?");
        }

        match sqlx::query(&query)
            .bind(charset) // 将字符集参数绑定到查询中
            .fetch_all(&self.pool)
            .await
        {
            Ok(rows) => {
                let collations: Vec<CollationInfo> = rows
                    .iter()
                    .map(|row| CollationInfo {
                        collation: row.get(0),
                        charset: row.get(1),
                        id: row.get(2),
                        is_default: row.get(3),
                        is_compiled: row.get(4),
                        // 根据实际情况添加其他属性
                    })
                    .collect();

                Response::new("Success", Some(collations))
            }
            Err(err) => Response::from_error(format!("Error: {}", err)),
        }
    }


    pub async fn get_table_create_statement(&self, database_name: &str, table_name: &str) -> Response<String> {
        let query = format!("SHOW CREATE TABLE {}.{}", database_name, table_name);

        match sqlx::query(&query).fetch_optional(&self.pool).await {
            Ok(row) => {
                if let Some(row) = row {
                    let create_statement: String = row.get(1);
                    Response::new("Success", Some(create_statement))
                } else {
                    Response::from_error("Table not found")
                }
            }
            Err(err) => Response::from_error(format!("Error: {}", err)),
        }
    }
    pub async fn get_table_columns_info(&self, database_name: &str, table_name: &str) -> Response<Vec<ColumnInfo>> {
        let query = format!(
            "SELECT COLUMN_NAME, DATA_TYPE, CHARACTER_MAXIMUM_LENGTH, NUMERIC_SCALE, IS_NULLABLE, COLUMN_KEY, COLUMN_COMMENT
                 FROM information_schema.COLUMNS
                 WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ?"
        );

        match sqlx::query(&query)
            .bind(database_name)
            .bind(table_name)
            .fetch_all(&self.pool)
            .await
        {
            Ok(rows) => {
                let columns_info: Vec<ColumnInfo> = rows
                    .iter()
                    .map(|row| {
                        ColumnInfo {
                            field: row.get(0),
                            ty: row.get(1),
                            length: row.try_get(2).ok(),
                            decimal: row.try_get(3).ok(),
                            is_null: row.get(4),
                            is_virtual: false, // 无法直接获取是否为虚拟字段，这里设置为 false
                            key: row.try_get(5).ok(),
                            comment: row.try_get(6).ok(),
                        }
                    })
                    .collect();

                Response::new("Success", Some(columns_info))
            }
            Err(err) => Response::from_error(format!("Error: {}", err)),
        }
    }

    pub async fn get_table_indexes_info(&self, database_name: &str, table_name: &str) -> Response<Vec<IndexInfo>> {
        let query = format!(
            "SELECT INDEX_NAME, COLUMN_NAME, INDEX_TYPE, INDEX_COMMENT
         FROM information_schema.STATISTICS
         WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ?"
        );

        match sqlx::query(&query)
            .bind(database_name)
            .bind(table_name)
            .fetch_all(&self.pool)
            .await
        {
            Ok(rows) => {
                let indexes_info: Vec<IndexInfo> = rows
                    .iter()
                    .map(|row| {
                        IndexInfo {
                            index_name: row.get(0),
                            column_name: row.get(1),
                            index_type: row.get(2),
                            index_comment: row.try_get(3).ok(),
                        }
                    })
                    .collect();

                Response::new("Success", Some(indexes_info))
            }
            Err(err) => Response::from_error(format!("Error: {}", err)),
        }
    }


    pub async fn get_table_foreign_keys_info(&self, database_name: &str, table_name: &str) -> Response<Vec<ForeignKeyInfo>> {
        let query = format!(
            "SELECT CONSTRAINT_NAME, COLUMN_NAME, REFERENCED_TABLE_SCHEMA, REFERENCED_TABLE_NAME, REFERENCED_COLUMN_NAME
             FROM information_schema.KEY_COLUMN_USAGE
             WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ? AND REFERENCED_TABLE_NAME IS NOT NULL"
        );


        let mut foreign_keys_info = Vec::new();
        let result = match sqlx::query(&query)
            .bind(database_name)
            .bind(table_name)
            .fetch_all(&self.pool)
            .await
        {
            Ok(rows) => {
                for row in rows.iter() {
                    let constraint_name: String = row.get(0);
                    let column_name: String = row.get(1);
                    let referenced_database: String = row.get(2);
                    let referenced_table_name: String = row.get(3);
                    let referenced_column_name: String = row.get(4);

                    // 查询外键的更新和删除规则
                    let query_constraints = format!(
                        "SELECT UPDATE_RULE, DELETE_RULE
                             FROM information_schema.REFERENTIAL_CONSTRAINTS
                             WHERE CONSTRAINT_NAME = ?"
                    );


                    let constraints_row = sqlx::query(&query_constraints)
                        .bind(&constraint_name)
                        .fetch_optional(&self.pool)
                        .await;

                    let (update_rule, delete_rule) = match constraints_row {
                        Ok(Some(row)) => (row.get::<Option<String>, _>(0), row.get::<Option<String>, _>(1)),
                        Ok(None) => (None, None),
                        Err(_) => (None, None),
                    };

                    foreign_keys_info.push(ForeignKeyInfo {
                        constraint_name,
                        column_name,
                        referenced_database,
                        referenced_table_name,
                        referenced_column_name,
                        update_rule,
                        delete_rule,
                    });
                }
                Response::new("Success", Some(foreign_keys_info))
            }
            Err(err) => {
                Response::from_error(format!("Error fetching foreign keys: {:?}", err))
            }
        };
        result
    }

    pub async fn get_detailed_table_info(
        &self,
        database_name: &str,
        table_name: &str,
    ) -> Response<DetailedTableInfo> {
        let query =
            format!(
                "SHOW TABLE STATUS from {}  like \"{}\"",
                database_name, table_name
            );
        let result = match sqlx::query(&query)
            .fetch_optional(&self.pool)
            .await
        {
            Ok(row) => {
                if let Some(row) = row {
                    let detailed_table_info =
                        DetailedTableInfo {
                            name: row.try_get("Name").ok(),
                            engine: row.try_get("Engine").ok(),
                            version: row.try_get("Version").ok(),
                            row_format: row.try_get("Row_format").ok(),
                            rows: row.try_get("Rows").ok(),
                            avg_row_length: row.try_get("Avg_row_length").ok(),
                            data_length: row.try_get("Data_length").ok(),
                            max_data_length: row.try_get("Max_data_length").ok(),
                            index_length: row.try_get("Index_length").ok(),
                            data_free: row.try_get("Data_free").ok(),
                            auto_increment: row.try_get("Auto_increment").ok(),
                            create_time: row.try_get("Create_time").ok(),
                            update_time: row.try_get("Update_time").ok(),
                            check_time: row.try_get("Check_time").ok(),
                            collation: row.try_get("Collation").ok(),
                            checksum: row.try_get("Checksum").ok(),
                            create_options: row.try_get("Create_options").ok(),
                            comment: row.try_get("Comment").ok(),
                        };

                    Response::new("Success", Some(detailed_table_info))
                } else {
                    Response::from_error("Table not found")
                }
            }
            Err(err) => {
                Response::from_error(format!("Error fetching table info: {:?}", err))
            }
        };

        result
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseInfo {
    pub version: String,
    pub status: Vec<StatusInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusInfo {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollationInfo {
    pub collation: String,
    pub charset: String,
    pub id: u64,
    pub is_default: String,
    pub is_compiled: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableDetails {
    pub create_table_statement: String,
    pub columns: Vec<ColumnInfo>,
    pub indexes: Vec<IndexInfo>,
    pub options: String,
    // 这里可以根据需要修改为更具体的类型
    pub comment: String,
    pub foreign_keys: Vec<ForeignKeyInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedTableInfo {
    pub name: Option<String>,
    pub engine: Option<String>,
    pub version: Option<i64>,
    pub row_format: Option<String>,
    pub rows: Option<i64>,
    pub avg_row_length: Option<i64>,
    pub data_length: Option<i64>,
    pub max_data_length: Option<i64>,
    pub index_length: Option<i64>,
    pub data_free: Option<i64>,
    pub auto_increment: Option<i64>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub check_time: Option<String>,
    pub collation: Option<String>,
    pub checksum: Option<String>,
    pub create_options: Option<String>,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexInfo {
    pub index_name: String,
    pub column_name: String,
    pub index_type: String,
    pub index_comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForeignKeyInfo {
    pub constraint_name: String,
    // 外键约束名称
    pub column_name: String,
    // 列名
    pub referenced_database: String,
    // 引用表所在的数据库名称
    pub referenced_table_name: String,
    // 引用表名称
    pub referenced_column_name: String,
    // 引用表列名
    pub update_rule: Option<String>,
    // 更新规则（可选）
    pub delete_rule: Option<String>,      // 删除规则（可选）
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnInfo {
    pub field: String,
    pub ty: String,
    pub length: Option<i32>,
    pub decimal: Option<i32>,
    pub is_null: String,
    pub is_virtual: bool,
    pub key: Option<String>,
    pub comment: Option<String>,
}