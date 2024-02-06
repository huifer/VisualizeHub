#[cfg(test)]
mod tests {
    use sqlx::mysql::MySqlConnectOptions;
    use sqlx::MySqlPool;

    #[tokio::test]
    async fn test_database_count() {
        let mysql_options = MySqlConnectOptions::new()
            .username("1")
            .password("")
            .host("")
            .port(1);

        let pool = MySqlPool::connect_with(mysql_options).await.unwrap();

        // 查询所有数据库的名称
        let databases: Vec<String> = sqlx::query_scalar("SHOW DATABASES")
            .fetch_all(&pool)
            .await
            .unwrap();

        // 在这里可以编写断言，验证预期的行为
        assert!(databases.len() > 0);
    }
}
