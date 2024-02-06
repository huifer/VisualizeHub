use core::iter::Map;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, Map as OtherMap, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeState {
    pub node_name: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vhost {
    #[serde(deserialize_with = "deserialize_cluster_state")]
    pub cluster_state: Vec<NodeState>,
    pub default_queue_type: String,
    pub description: String,
    pub metadata: Metadata,
    pub name: String,
    pub tags: Vec<String>,
    pub tracing: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub description: String,
    pub tags: Vec<String>,
}

fn deserialize_cluster_state<'de, D>(deserializer: D) -> Result<Vec<NodeState>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let state_map: OtherMap<String, Value> = Deserialize::deserialize(deserializer)?;
    Ok(state_map
        .into_iter()
        .map(|(node_name, state)| NodeState {
            node_name,
            state: state.as_str().unwrap_or_default().to_string(),
        })
        .collect())
}
impl Vhost {
    pub fn from_json(json_str: &str) -> Result<Vec<Vhost>, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}

#[test]
fn test_deserialize_vhost() {
    // JSON数据
    let json_str = r#"
   [{"cluster_state":{"rabbit@f6ec723fb48c":"running"},"default_queue_type":"undefined","description":"Default virtual host","metadata":{"description":"Default virtual host","tags":[]},"name":"/","tags":[],"tracing":false}]
    "#;

    let vhosts: Vec<Vhost> = from_str(json_str).expect("Failed to deserialize JSON");
    println!("{:?}", vhosts);
}
