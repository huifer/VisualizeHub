use crate::rabbitmq_info::RabbitMQInfo;
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

    pub async fn list_vhosts(&self) -> Result<Vec<Vhost>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let api_url = format!(
            "http://{}:{}/api/vhosts",
            self.rabbitmq_info.host, self.rabbitmq_info.port
        );

        let request = client.get(&api_url).basic_auth(
            &self.rabbitmq_info.username,
            Some(&self.rabbitmq_info.password),
        );

        let response = request.send().await?;

        // Ensure the request was successful (2xx status code)
        let response = response.error_for_status()?;

        // Parse the JSON response into a string
        let body = response.text().await?;

        println!("body = {}", body);
        // Parse the JSON string into a Vhost struct
        let vhost = Vhost::from_json(&body);

        match vhost {
            Ok(ref vhosts) => Ok(vhost?),
            Err(e) => Ok(Vec::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rabbitmq_info::RabbitMQInfo;
    use crate::vhosts_manager::VhostsManager;

    #[tokio::test]
    async fn test_list_vhosts() {
        // Replace these values with your actual RabbitMQ information for testing
        let rabbitmq_info = RabbitMQInfo {
            username: String::from("guest"),
            password: String::from("guest"),
            host: String::from("127.0.0.1"),
            port: 15672,
        };

        // Create a VhostsManager instance with the test RabbitMQInfo
        let vhosts_manager = VhostsManager::new(rabbitmq_info);

        let _result = vhosts_manager.list_vhosts().await.unwrap();
        println!("_result = {:?}", _result)
    }
}
