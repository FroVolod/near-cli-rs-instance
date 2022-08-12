#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub credentials_home_dir: std::path::PathBuf,
    pub networks: std::collections::HashMap<String, NetworkConfig>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NetworkConfig {
    pub network_name: String,
    pub rpc_url: url::Url,
    pub wallet_url: url::Url,
    pub explorer_transaction_url: url::Url,
    pub api_key: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        let home_dir = dirs::home_dir().expect("Impossible to get your home dir!");
        let mut credentials_home_dir = std::path::PathBuf::from(&home_dir);
        credentials_home_dir.push(".near-credentials");

        let mut networks = std::collections::HashMap::new();
        networks.insert(
            "mainnet".to_string(),
            NetworkConfig {
                network_name: "mainnet".to_string(),
                rpc_url: "https://archival-rpc.mainnet.near.org".parse().unwrap(),
                wallet_url: "https://wallet.mainnet.near.org".parse().unwrap(),
                explorer_transaction_url: "https://explorer.mainnet.near.org/transactions/"
                    .parse()
                    .unwrap(),
                api_key: None,
            },
        );
        networks.insert(
            "testnet".to_string(),
            NetworkConfig {
                network_name: "testnet".to_string(),
                rpc_url: "https://archival-rpc.testnet.near.org".parse().unwrap(),
                wallet_url: "https://wallet.testnet.near.org".parse().unwrap(),
                explorer_transaction_url: "https://explorer.testnet.near.org/transactions/"
                    .parse()
                    .unwrap(),
                api_key: None,
            },
        );
        networks.insert(
            "betanet".to_string(),
            NetworkConfig {
                network_name: "betanet".to_string(),
                rpc_url: "https://rpc.betanet.near.org".parse().unwrap(),
                wallet_url: "https://wallet.betanet.near.org".parse().unwrap(),
                explorer_transaction_url: "https://explorer.betanet.near.org/transactions/"
                    .parse()
                    .unwrap(),
                api_key: None,
            },
        );
        Self {
            credentials_home_dir,
            networks,
        }
    }
}

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
