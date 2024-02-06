#[cfg(test)]
mod tests {
    use elastic::prelude::*;

    use db_show::op::es_op::EsOperation;

    #[tokio::test]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Elasticsearch节点的URL
        let url = "1";

        // 创建Elasticsearch连接
        let conn_result = SyncClientBuilder::new().static_node(url).build();

        // 使用 unwrap 或者 expect 从 Result 中提取成功值
        let conn = conn_result?;

        // 执行你的Elasticsearch操作，这里只是一个示例
        let response: PingResponse = conn
            .ping()
            .send()
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;

        // 确保ping成功
        println!("{:?}", response);
        Ok(())
    }

    #[tokio::test]
    async fn test() -> Result<(), Box<dyn std::error::Error>> {
        // Elasticsearch节点的URL
        let url = "1";
        let c = EsOperation::new(url.to_string());
        // 获取集群健康状态
        let x = c.get_cluster_health().await;

        dbg!(x);
        let x1 = c.get_nodes_stats().await;
        dbg!(x1);
        let x12 = c.get_index_stats().await;
        dbg!(x12);

        // // 获取索引状态
        // let indices_url = format!("{}/_cat/indices", url);
        // let indices_status = reqwest::get(&indices_url).await?.text().await?;
        //
        // println!("Indices Status: {}", indices_status);
        //
        // // 其他监控信息...

        Ok(())
    }
}
