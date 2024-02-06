use crate::rabbitmq_info::RabbitMQInfo;
use reqwest::Client;

#[derive(Debug)]
pub struct ExchangeManager {
    rabbitmq_info: RabbitMQInfo,
}

impl ExchangeManager {
    fn new(info: RabbitMQInfo) -> Self {
        ExchangeManager {
            rabbitmq_info: info,
        }
    }

    fn list_exchanges(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_USERNAME: &str = "guest";
    const MOCK_PASSWORD: &str = "guest";
    const MOCK_HOST: &str = "localhost";
    const MOCK_PORT: i32 = 15672;

    fn create_mock_rabbitmq_info() -> RabbitMQInfo {
        RabbitMQInfo::new(MOCK_USERNAME, MOCK_PASSWORD, MOCK_HOST, MOCK_PORT)
    }

    #[test]
    fn test_exchange_manager_list_exchanges() {
        // Create a mock RabbitMQInfo instance
        let rabbitmq_info = create_mock_rabbitmq_info();

        // Create an ExchangeManager instance
        let exchange_manager = ExchangeManager::new(rabbitmq_info);
        // exchange_manager.list_exchanges();
    }
}
