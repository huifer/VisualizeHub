use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MongoServerInfo {
    pub host: String,
    pub version: String,
    pub process: String,
    pub pid: i64,
    pub uptime: f64,
    pub uptime_millis: i64,
    pub uptime_estimate: i64,
}

/// 结构体表示 MongoDB 服务器的内存信息
#[derive(Deserialize, Serialize, Debug)]
pub struct MongoMemInfo {
    /// MongoDB 服务器运行的位数
    pub bits: i32,

    /// MongoDB 进程当前占用的物理内存大小（以兆字节为单位）
    pub resident: i32,

    /// MongoDB 进程的虚拟内存大小（以兆字节为单位）
    #[serde(rename = "virtual")]
    pub virtual_memory: i32,

    /// 服务器是否支持内存统计信息
    pub supported: bool,
}

///  MongoDB 服务器的连接信息
#[derive(Deserialize, Serialize, Debug)]
pub struct MongoConnectionsInfo {
    /// 当前活动连接数
    pub current: i32,

    /// 可用连接数
    pub available: i32,

    /// 总创建的连接数
    pub total_created: i32,

    /// 被拒绝的连接数
    pub rejected: i32,

    /// 当前活跃的连接数
    pub active: i32,

    /// 线程连接数
    pub threaded: i32,

    /// 消耗 IsMaster 操作的连接数
    pub exhaust_is_master: i32,

    /// 消耗 Hello 操作的连接数
    pub exhaust_hello: i32,

    /// 等待拓扑变化的连接数
    pub awaiting_topology_changes: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MongoServerInfoCol {
    pub mongo_server_info: MongoServerInfo,
    pub mongo_mem_info: MongoMemInfo,
    pub mongo_connections_info: MongoConnectionsInfo,
}
