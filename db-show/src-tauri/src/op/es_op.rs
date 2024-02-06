use crate::op::es_entity::ClusterHealth;
use crate::resp::resp::Response;

pub struct EsOperation {
    pub url: String,
}

impl EsOperation {
    pub async fn get_index_stats(&self) -> Response<String> {
        let nodes_stats_url = format!("{}/_cat/indices", self.url);

        match reqwest::get(&nodes_stats_url).await {
            Ok(response) => {
                let status_code = response.status().as_u16() as i32;
                let description = if response.status().is_success() {
                    "操作成功".to_string()
                } else {
                    format!("操作失败，HTTP状态码: {}", status_code)
                };

                // 获取节点统计信息

                let nodes_stats = response.text().await.unwrap_or_else(|_| String::new());

                Response::new(description, Some(nodes_stats))
            }
            Err(err) => Response::from_error(format!("请求失败: {}", err)),
        }
    }

    pub async fn get_nodes_stats(&self) -> Response<String> {
        let nodes_stats_url = format!("{}/_nodes/stats", self.url);

        match reqwest::get(&nodes_stats_url).await {
            Ok(response) => {
                let status_code = response.status().as_u16() as i32;
                let description = if response.status().is_success() {
                    "操作成功".to_string()
                } else {
                    format!("操作失败，HTTP状态码: {}", status_code)
                };

                // 获取节点统计信息

                let nodes_stats = response.text().await.unwrap_or_else(|_| String::new());

                Response::new(description, Some(nodes_stats))
            }
            Err(err) => Response::from_error(format!("请求失败: {}", err)),
        }
    }
    pub async fn get_cluster_health(&self) -> Response<ClusterHealth> {
        let cluster_health_url = format!("{}/_cluster/health", self.url);

        match reqwest::get(&cluster_health_url).await {
            Ok(response) => {
                let status_code = response.status().as_u16() as i32;
                let description = if response.status().is_success() {
                    "操作成功".to_string()
                } else {
                    format!("操作失败，HTTP状态码: {}", status_code)
                };
                let cluster_health = response.text().await.unwrap_or_else(|_| String::new());

                let cluster_health: ClusterHealth =
                    serde_json::from_str(cluster_health.as_str()).unwrap();

                Response::new(description, Some(cluster_health))
            }
            Err(err) => Response::from_error(format!("请求失败: {}", err)),
        }
    }
    pub fn new(string: String) -> Self {
        Self { url: string }
    }
}
