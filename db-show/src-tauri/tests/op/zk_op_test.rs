use db_show::op::zk_op::ZookeeperOperation;

/// feature：创建节点并设置数据
#[tokio::test]
async fn main() {
    // // 连接到 ZooKeeper 服务器
    // let (zk, default_watcher) = ZooKeeper::connect(&"127.0.0.1:2181".parse().unwrap())
    //     .await
    //     .unwrap();
    //
    // // 准备要设置的字符串数据
    // let string_data = String::from("Hello, world!");
    // let bytes_data: Vec<u8> = string_data.into_bytes(); // 将数据转换为 Vec<u8>
    //
    // // 创建节点并设置数据
    // let path = zk
    //     .create(
    //         "/example/b",         // 节点路径
    //         bytes_data,            // 数据
    //         Acl::open_unsafe(),    // 使用不安全的 ACL（Access Control List）
    //         CreateMode::Persistent, // 永久性节点
    //     )
    //     .await
    //     .unwrap();
    let operation = ZookeeperOperation::new("127.0.0.1:2181".to_string());
    let x = operation.create_and_set_data("/b".to_string(), "aa".to_string()).await;
    dbg!(x.unwrap());
}

/// feature：获取节点状态
#[tokio::test]
async fn status() {
    let operation = ZookeeperOperation::new("127.0.0.1:2181".to_string());
    let x = operation.get_zookeeper_data("/example/b").await;
    dbg!(x.unwrap());
}

/// 设置 ZooKeeper 节点数据
#[tokio::test]
async fn set_zookeeper_data() {
    let operation = ZookeeperOperation::new("127.0.0.1:2181".to_string());
    let result = operation.set_zookeeper_data("/example".to_string(), "jklas".to_string()).await;
}

/// feature: 查询子节点
#[tokio::test]
async fn main3() {
    let operation = ZookeeperOperation::new("127.0.0.1:2181".to_string());
    let result = operation.get_children_of_parent("/").await;

    dbg!(result.unwrap());
}