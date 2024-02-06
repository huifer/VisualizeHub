use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct VhostPermission {
    pub vhost: String,
    pub user: String,
    pub configure: String,
    pub write: String,
    pub read: String,
}

impl VhostPermission {
    pub fn from_json(json_str: &str) -> Result<Vec<VhostPermission>, serde_json::Error> {
        let parsed_data: Vec<VhostPermission> = serde_json::from_str(json_str)?;
        Ok(parsed_data)
    }
}

#[test]
fn from_json() {
    let json_data = r#"[{"vhost":"foo","user":"guest","configure":".*","write":".*","read":".*"}]"#;

    let result = VhostPermission::from_json(json_data);
    // 调用函数解析 JSON 字符串
    match result {
        Ok(parsed_data) => {
            // 打印解析后的数据
            for entry in parsed_data {
                println!("{:?}", entry);
            }
        }
        Err(e) => {
            // 打印解析错误
            eprintln!("Failed to parse JSON: {}", e);
        }
    }
}
