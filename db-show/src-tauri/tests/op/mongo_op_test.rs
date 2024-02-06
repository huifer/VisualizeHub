#[cfg(test)]
mod tests {
    use db_show::config::mongo_config::MongoUserPassword;
    use db_show::op::mongo_op::MongoOperation;

    #[tokio::test]
    async fn tests() {
        let operation = get_mongo_op().await;

        // 获取数据库列表
        let databases = operation.db_names().await;
        dbg!(databases);
        let result = operation.get_server_info().await;
        dbg!(result);
    }

    async fn get_mongo_op() -> MongoOperation {
        let mongo_user_password = MongoUserPassword {
            username: "admin".to_string(),
            password: "admin".to_string(),
            host: "1".to_string(),
            port: 27817,
            name: "aa".to_string(),
        };

        let operation = MongoOperation::new(&mongo_user_password).await.unwrap();
        operation
    }

    #[tokio::test]
    async fn test_collection_names() {
        let operation = get_mongo_op().await;
        let x = operation.collection_names("local").await;
        dbg!(x);
    }

    #[tokio::test]
    async fn test_get_collection_indexes() {
        let operation = get_mongo_op().await;
        let x = operation.get_collection_indexes("local", "cc").await;
        dbg!(x);
    }

    #[tokio::test]
    async fn test_cd() {
        let operation = get_mongo_op().await;
        // let x2 = operation.create_collection("gogo", "coco").await;
        // dbg!(x2);
        let x3 = operation.drop_collection("gogo", "coco").await;
        dbg!(x3);
        // let x4 = operation.drop_database("gogo").await;
        // dbg!(x4);
    }
}
