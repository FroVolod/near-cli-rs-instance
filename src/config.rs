#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    networks: std::collections::HashMap<String, NetworkConfig>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NetworkConfig {
    url: url::Url,
    api_key: Option<String>,
}
