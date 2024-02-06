use env_logger;
use log::debug;
use reqwest::Client;
use serde_json::json;

use crate::rabbitmq_info::RabbitMQInfo;
use crate::resp::vhost_permissions::VhostPermission;
use crate::resp::vhosts_list_resp::Vhost;

pub struct VhostsManager {
    pub rabbitmq_info: RabbitMQInfo,
}

impl VhostsManager {
    fn new(info: RabbitMQInfo) -> Self {
        VhostsManager {
            rabbitmq_info: info,
        }
    }

    /// 获取虚拟主机列表
    pub async fn list_vhosts(&self) -> Result<Vec<Vhost>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let api_url = format!(
            "http://{}:{}/api/vhosts",
            self.rabbitmq_info.host, self.rabbitmq_info.port
        );
        debug!("api url = {}", api_url);

        let request = client.get(&api_url).basic_auth(
            &self.rabbitmq_info.username,
            Some(&self.rabbitmq_info.password),
        );

        let response = request.send().await?;

        let response = response.error_for_status()?;

        let body = response.text().await?;

        debug!("body = {}", body);
        // Parse the JSON string into a Vhost struct
        let vhost = Vhost::from_json(&body);

        match vhost {
            Ok(ref vhosts) => Ok(vhost?),
            Err(e) => Ok(Vec::new()),
        }
    }

    /// 创建虚拟机
    ///
    /// # Arguments
    ///
    /// * `vhost_name`:
    /// * `description`:
    /// * `tags`:
    /// * `default_queue_type`:   classic , quorum , stream
    ///
    /// returns: Result<(), Box<dyn Error, Global>>
    ///
    pub async fn create_vhost(
        &self,
        name: &str,
        description: &str,
        tags: &str,
        default_queue_type: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();

        let api_url = format!(
            "http://{}:{}/api/vhosts/{}",
            self.rabbitmq_info.host, self.rabbitmq_info.port, name
        );

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);

        let data = json!({
            "name": name,
            "description": description,
            "tags": tags,
            "default_queue_type": default_queue_type
        });

        let response = client
            .put(&api_url)
            .basic_auth(
                &self.rabbitmq_info.username,
                Some(&self.rabbitmq_info.password),
            )
            .headers(headers)
            .json(&data)
            .send()
            .await?;
        let body = response.text().await?;
        debug!("body = {}", body);
        Ok(())
    }

    pub async fn permissions(
        &self,
        name: &str,
    ) -> Result<Vec<VhostPermission>, Box<dyn std::error::Error>> {
        let client = Client::new();

        let api_url = format!(
            "http://{}:{}/api/vhosts/{}/permissions",
            self.rabbitmq_info.host, self.rabbitmq_info.port, name
        );

        let response = client
            .get(&api_url)
            .basic_auth(
                &self.rabbitmq_info.username,
                Some(&self.rabbitmq_info.password),
            )
            .send()
            .await?;
        let body = response.text().await?;
        debug!("body = {}", body);
        let result = VhostPermission::from_json(&body);
        debug!("result = {:?}", result);

        match result {
            Ok(permission) => Ok(permission),
            Err(e) => Ok(Vec::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use env_logger::Builder;
    use log::LevelFilter;

    use crate::rabbitmq_info::RabbitMQInfo;
    use crate::vhosts_manager::VhostsManager;

    #[tokio::test]
    async fn test_list_vhosts() {
        Builder::from_default_env()
            .filter(None, LevelFilter::Debug)
            .init();
        let rabbitmq_info = mock();

        let vhosts_manager = VhostsManager::new(rabbitmq_info);

        let _result = vhosts_manager.list_vhosts().await.unwrap();
        println!("_result = {:?}", _result)
    }

    #[tokio::test]
    async fn create_vhost() {
        Builder::from_default_env()
            .filter(None, LevelFilter::Debug)
            .init();
        let rabbitmq_info = mock();

        let vhosts_manager = VhostsManager::new(rabbitmq_info);
        let vhost_name = "foo";

        match vhosts_manager
            .create_vhost(vhost_name, "测试", "a", "classic")
            .await
        {
            Ok(()) => println!("Virtual host '{}' created successfully", vhost_name),
            Err(e) => eprintln!("Error creating virtual host: {:?}", e),
        }
    }

    #[tokio::test]
    async fn permission() {
        Builder::from_default_env()
            .filter(None, LevelFilter::Debug)
            .init();
        let rabbitmq_info = mock();

        let vhosts_manager = VhostsManager::new(rabbitmq_info);
        let vhost_name = "foo";

        match vhosts_manager.permissions(vhost_name).await {
            Ok(permis) => println!("{}", permis.len()),
            Err(e) => eprintln!("Error creating virtual host: {:?}", e),
        }
    }

    fn mock() -> RabbitMQInfo {
        let rabbitmq_info = RabbitMQInfo {
            username: String::from("guest"),
            password: String::from("guest"),
            host: String::from("127.0.0.1"),
            port: 15672,
        };
        rabbitmq_info
    }
}
