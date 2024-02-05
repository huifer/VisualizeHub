use serde::{Deserialize, Serialize};
use tokio_zookeeper::{Acl, CreateMode, ZooKeeper};

pub struct ZookeeperOperation {
    pub url: String,
}

impl ZookeeperOperation {
    pub fn new(url: String) -> Self {
        Self {
            url
        }
    }

    /// 设置 ZooKeeper 数据
    ///
    /// # 参数
    ///
    /// * `path`: 节点路径
    /// * `string_data`: 字符串形式的数据
    ///
    /// 返回: Result<(), Box<dyn Error>>
    ///
    ///
    pub async fn set_zookeeper_data(&self, path: String, string_data: String) -> Result<(), Box<dyn std::error::Error>> {
        // 连接到 ZooKeeper 服务器
        let (zk, _) = ZooKeeper::connect(&self.url.parse()?) // 使用?操作符传播错误
            .await?;

        // 获取节点数据
        let res = zk.get_data(path.as_str()).await?;
        let res = res.unwrap(); // 这里需要注意处理 res 为 None 的情况

        // 新的数据
        let bytes_data: Vec<u8> = string_data.into_bytes(); // 将数据转换为 Vec<u8>

        // 设置节点数据，通过移动所有权避免生命周期问题
        let stat = zk
            .set_data(path.as_str(), Some(res.1.version), bytes_data)
            .await?;

        Ok(())
    }

    /// 获取父节点下的所有子节点
    ///
    /// # 参数
    ///
    /// * `parent_path`: 父节点路径
    ///
    /// 返回: Result<Option<Vec<String>>, Box<dyn std::error::Error>>
    pub async fn get_children_of_parent(&self, parent_path: &str) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        // 连接到 ZooKeeper 服务器
        let (zk, _) = ZooKeeper::connect(&self.url.parse()?)
            .await?;

        // 获取父节点下的所有子节点
        let children = zk.get_children(parent_path).await?;

        Ok(children)
    }

    /// 获取 ZooKeeper 节点数据及其状态信息
    ///
    /// # 参数
    ///
    /// * `path`: 节点路径
    ///
    /// 返回: Result<ZookeeperData, Box<dyn std::error::Error>>
    pub async fn get_zookeeper_data(&self, path: &str) -> Result<ZookeeperData, Box<dyn std::error::Error>> {
        // 连接到 ZooKeeper 服务器
        let (zk, default_watcher) = ZooKeeper::connect(&self.url.parse()?)
            .await?;

        // 获取节点数据及其状态信息
        let result = zk.get_data(path).await?;

        match result {
            Some((data, stat)) => {
                Ok(ZookeeperData {
                    data: String::from_utf8_lossy(&data).to_string(),
                    status: ZookeeperStatus {
                        czxid: stat.czxid,
                        mzxid: stat.mzxid,
                        ctime: stat.ctime,
                        mtime: stat.mtime,
                        version: stat.version,
                        cversion: stat.cversion,
                        aversion: stat.aversion,
                        ephemeral_owner: stat.ephemeral_owner,
                        data_length: stat.data_length,
                        num_children: stat.num_children,
                        pzxid: stat.pzxid,
                    },
                })
            }
            None => {
                Err("Node does not exist".into())
            }
        }
    }
    /// 创建 ZooKeeper 节点并设置数据
    ///
    /// # 参数
    ///
    /// * `path`: 节点路径
    /// * `string_data`: 字符串形式的数据
    ///
    /// 返回: Result<String, Box<dyn std::error::Error>>
    pub async fn create_and_set_data(&self, path: String, string_data: String) -> Result<String, Box<dyn std::error::Error>> {
        // 连接到 ZooKeeper 服务器
        let (zk, default_watcher) = ZooKeeper::connect(&self.url.parse()?)
            .await?;

        // 对参数进行控制 FIXME: 这里可能需要进一步的参数验证

        let bytes_data: Vec<u8> = string_data.into_bytes();
        let created_path = zk
            .create(
                path.as_str(),         // 节点路径
                bytes_data,            // 数据
                Acl::open_unsafe(),    // 使用不安全的 ACL（Access Control List）
                CreateMode::Persistent, // 永久性节点
            )
            .await?;

        Ok(created_path)
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ZookeeperData {
    pub data: String,
    pub status: ZookeeperStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZookeeperStatus {
    pub czxid: i64,
    pub mzxid: i64,
    pub ctime: i64,
    pub mtime: i64,
    pub version: i32,
    pub cversion: i32,
    pub aversion: i32,
    pub ephemeral_owner: i64,
    pub data_length: i32,
    pub num_children: i32,
    pub pzxid: i64,
}

