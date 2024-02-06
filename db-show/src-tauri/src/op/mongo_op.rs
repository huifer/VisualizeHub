use futures_lite::StreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::ClientOptions;
use mongodb::{Client, IndexModel};

use crate::config::mongo_config::MongoUserPassword;
use crate::op::mongo_entity::{
    MongoConnectionsInfo, MongoMemInfo, MongoServerInfo, MongoServerInfoCol,
};
use crate::resp::resp::Response;

pub struct MongoOperation {
    client: Client,
}

impl MongoOperation {
    pub async fn drop_database(&self, database_name: &str) -> Response<bool> {
        match self.client.database(database_name).drop(None).await {
            Ok(_) => Response::new("删除数据库成功", Some(true)),
            Err(err) => Response::from_error(format!("删除数据库时出错: {}", err)),
        }
    }

    pub async fn create_collection(
        &self,
        database_name: &str,
        collection_name: &str,
    ) -> Response<bool> {
        match self
            .client
            .database(database_name)
            .create_collection(collection_name, None)
            .await
        {
            Ok(_) => Response::new("创建建collection成功", Some(true)),
            Err(err) => Response::from_error(format!("创建集合时出错: {}", err)),
        }
    }

    pub async fn drop_collection(
        &self,
        database_name: &str,
        collection_name: &str,
    ) -> Response<bool> {
        match self
            .client
            .database(database_name)
            .collection::<Document>(collection_name)
            .drop(None)
            .await
        {
            Ok(_) => Response::new("删除集合成功", Some(true)),
            Err(err) => Response::from_error(format!("删除集合时出错: {}", err)),
        }
    }
    pub async fn get_collection_indexes(
        &self,
        database_name: &str,
        collection_name: &str,
    ) -> Response<Vec<IndexModel>> {
        let collection: mongodb::Collection<Document> = self
            .client
            .database(database_name)
            .collection(collection_name);

        match collection.list_indexes(None).await {
            Ok(mut cursor) => {
                let mut indexes = Vec::new();
                while let Some(doc) = cursor.next().await {
                    if let Ok(index_document) = doc {
                        indexes.push(index_document);
                    }
                }
                Response::new("操作成功", Some(indexes))
            }
            Err(err) => Response::from_error(format!("获取索引时出错: {}", err)),
        }
    }

    pub async fn collection_names(&self, database_name: &str) -> Response<Vec<String>> {
        let db = self.client.database(database_name);

        match db.list_collection_names(None).await {
            Ok(collections) => Response::new("Operation successful", Some(collections)),
            Err(err) => Response::from_error(format!("Error listing collections: {}", err)),
        }
    }
    pub async fn get_server_info(&self) -> Response<MongoServerInfoCol> {
        let admin_db = self.client.database("admin");
        let command = doc! {
            "serverStatus": 1,
            "recordStats": 0,
            "repl": 0,
            "metrics": 1,
            "locks": 0,
            "backgroundFlushing": 0,
            "asserts": 0,
            "connections": 1,
            "extra_info": 0,
            "index_counters": 0,
            "network": 1,
            "opcounters": 0,
            "opcountersRepl": 0,
            "mem": 1,
            "wiredTiger": 0,
            "ft": 0,
            "uptime": 1,
            "uptimeEstimate": 1,
        };

        if let Ok(result) = admin_db.run_command(command, None).await {
            // dbg!(&result);
            // 获取服务器信息中的某个字段，这里假设你想获取 "version" 字段
            let server_info = MongoServerInfo {
                host: result.get_str("host").unwrap().to_string(),
                version: result.get_str("version").unwrap().to_string(),
                process: result.get_str("process").unwrap().to_string(),
                pid: result.get_i64("pid").unwrap(),
                uptime: result.get_f64("uptime").unwrap(),
                uptime_millis: result.get_i64("uptimeMillis").unwrap(),
                uptime_estimate: result.get_i64("uptimeEstimate").unwrap(),
            };
            let mem_document = result.get_document("mem").unwrap();
            let mem_info = MongoMemInfo {
                bits: mem_document.get_i32("bits").unwrap(),
                resident: mem_document.get_i32("resident").unwrap(),
                virtual_memory: mem_document.get_i32("virtual").unwrap(),
                supported: mem_document.get_bool("supported").unwrap(),
            };
            let connection_document = result.get_document("connections").unwrap();
            let conn_info = MongoConnectionsInfo {
                current: connection_document.get_i32("current").unwrap(),
                available: connection_document.get_i32("available").unwrap(),
                total_created: connection_document.get_i32("totalCreated").unwrap(),
                rejected: connection_document.get_i32("rejected").unwrap(),
                active: connection_document.get_i32("active").unwrap(),
                threaded: connection_document.get_i32("threaded").unwrap(),
                exhaust_is_master: connection_document.get_i32("exhaustIsMaster").unwrap(),
                exhaust_hello: connection_document.get_i32("exhaustHello").unwrap(),
                awaiting_topology_changes: connection_document
                    .get_i32("awaitingTopologyChanges")
                    .unwrap(),
            };
            let r = MongoServerInfoCol {
                mongo_server_info: server_info,
                mongo_mem_info: mem_info,
                mongo_connections_info: conn_info,
            };
            return Response::new("Server info retrieved successfully", Some(r));
        };

        // 如果获取信息失败或无法找到期望的字段，返回错误信息
        Response::from_error("Failed to retrieve server info")
    }
    pub async fn db_names(&self) -> Response<Vec<String>> {
        match self.client.list_database_names(None, None).await {
            Ok(databases) => Response::new("Operation successful", Some(databases)),
            Err(err) => Response::from_error(format!("Error listing database names: {}", err)),
        }
    }
    pub async fn new(
        mongo_user_password: &MongoUserPassword,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mongodb_url =
            if mongo_user_password.password.is_empty() && mongo_user_password.username.is_empty() {
                format!(
                    "mongodb://{}:{}/",
                    mongo_user_password.host, mongo_user_password.port
                )
            } else {
                format!(
                    "mongodb://{}:{}@{}:{}/",
                    mongo_user_password.username,
                    mongo_user_password.password,
                    mongo_user_password.host,
                    mongo_user_password.port
                )
            };

        println!("mongo url = {}", mongodb_url);
        // 创建客户端选项
        let mut client_options = ClientOptions::parse(&mongodb_url).await.unwrap();

        // 设置连接池的最大大小
        client_options.max_pool_size = Some(5);

        // 连接MongoDB
        let client = Client::with_options(client_options).unwrap();
        Ok(Self { client })
    }
}
