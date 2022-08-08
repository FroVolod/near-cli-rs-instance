#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub networks: std::collections::HashMap<String, NetworkConfig>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NetworkConfig {
    pub url: url::Url,
    api_key: Option<String>,
}

// impl Default for Config {
//     fn default() -> Self {
//         let mut networks = std::collections::HashMap::new();
//         networks.insert("mainnet", NetworkConfig {
//             rpc_url: url::Url::new("https://archival-rpc.mainnet.near.org").unwrap(),
//             // wallet_url
//             // explorer_transaction_url
//             api_key: None,
//         });
//         networks.insert("testnet", NetworkConfig {
//             url: url::Url::new("https://archival-rpc.testnet.near.org").unwrap(),
//             api_key: None,
//         });
//          networks.insert("localnet", NetworkConfig {
//             url: url::Url::new("http://127.0.0.1:3030").unwrap(),
//             api_key: None,
//         });
//         Self {
//             networks
//         }
//     }
// }

// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
// pub struct Config {
//     credentials_home_dir: std::path::PathBuf, // <HOME>/.near-credentials + <network.network_name>
//     networks: std::collections::HashMap<String, NetworkConfig>,
// }

// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
// pub struct NetworkConfig {
//     network_name: Option<String>, // mainnet
//     url: url::Url,
//     api_key: Option<String>,
// }


// r#"
//         credentials_home_dir = "~/.near-credentials/" // + <chain_id>

//         [networks.mainnet-qbit]
//         chain_id = "mainnet"
//         url = "https://mainnet.near.qbit.org.ua"

//         [networks.mainnet-pagoda]
//         chain_id = "mainnet"
//         url = "https://rpc.mainnet.pagoda.co"
 
//         [networks.testnet]
//         url = "https://archival-rpc.testnet.near.org"

//         [networks.localnet]
//         url = "http://127.0.0.1:3030"
//     "#,
