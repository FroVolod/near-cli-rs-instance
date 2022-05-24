extern crate dirs;

use serde::Deserialize;

#[derive(clap::Args, Debug, Clone)]
pub struct SignKeychain {
    nonce: Option<u64>,
    block_hash: Option<String>,
    #[clap(subcommand)]
    pub submit: super::Submit,
}

#[derive(Debug, Deserialize)]
struct User {
    account_id: near_primitives::types::AccountId,
    public_key: near_crypto::PublicKey,
    private_key: near_crypto::SecretKey,
}

impl SignKeychain {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: crate::common::ConnectionConfig,
    ) -> crate::CliResult {
        let home_dir = dirs::home_dir().expect("Impossible to get your home dir!");
        let file_name = format!("{}.json", prepopulated_unsigned_transaction.signer_id);
        let mut path = std::path::PathBuf::from(&home_dir);

        let data_path: std::path::PathBuf = {
            let dir_name = network_connection_config.dir_name();
            path.push(dir_name);
            path.push(file_name);

            if path.exists() {
                path
            } else {
                let query_view_method_response = near_jsonrpc_client::JsonRpcClient::connect(
                    &network_connection_config.rpc_url().as_str(),
                )
                .call(near_jsonrpc_client::methods::query::RpcQueryRequest {
                    block_reference: near_primitives::types::Finality::Final.into(),
                    request: near_primitives::views::QueryRequest::ViewAccessKeyList {
                        account_id: prepopulated_unsigned_transaction.signer_id.clone(),
                    },
                })
                .await
                .map_err(|err| {
                    color_eyre::Report::msg(format!(
                        "Failed to fetch query for view key list: {:?}",
                        err
                    ))
                })?;
                let access_key_view =
                    if let near_jsonrpc_primitives::types::query::QueryResponseKind::AccessKeyList(
                        result,
                    ) = query_view_method_response.kind
                    {
                        result
                    } else {
                        return Err(color_eyre::Report::msg(format!("Error call result")));
                    };
                let mut path = std::path::PathBuf::from(&home_dir);
                path.push(dir_name);
                path.push(&prepopulated_unsigned_transaction.signer_id.to_string());
                let mut data_path = std::path::PathBuf::new();
                'outer: for access_key in access_key_view.keys {
                    let account_public_key = access_key.public_key.to_string();
                    let is_full_access_key: bool = match &access_key.access_key.permission {
                        near_primitives::views::AccessKeyPermissionView::FullAccess => true,
                        near_primitives::views::AccessKeyPermissionView::FunctionCall {
                            allowance: _,
                            receiver_id: _,
                            method_names: _,
                        } => false,
                    };
                    let dir = path
                            .read_dir()
                            .map_err(|err| {
                                color_eyre::Report::msg(format!("There are no access keys found in the keychain for the signer account. Log in before signing transactions with keychain. {}", err))
                            })?;
                    for entry in dir {
                        if let Ok(entry) = entry {
                            if entry
                                .path()
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .contains(account_public_key.rsplit(':').next().unwrap())
                                && is_full_access_key
                            {
                                data_path.push(entry.path());
                                break 'outer;
                            }
                        } else {
                            return Err(color_eyre::Report::msg(format!(
                                    "There are no access keys found in the keychain for the signer account. Log in before signing transactions with keychain."
                                )));
                        };
                    }
                }
                data_path
            }
        };
        let data = std::fs::read_to_string(data_path).map_err(|err| {
            color_eyre::Report::msg(format!("Access key file not found! Error: {}", err))
        })?;
        let account_json: User = serde_json::from_str(&data)
            .map_err(|err| color_eyre::Report::msg(format!("Error reading data: {}", err)))?;
        let sign_with_private_key = super::sign_with_private_key::SignPrivateKey {
            signer_public_key: account_json.public_key,
            signer_private_key: account_json.private_key,
            nonce: self.nonce.clone(),
            block_hash: self.block_hash.clone(),
            submit: self.submit.clone(),
        };
        sign_with_private_key
            .process(prepopulated_unsigned_transaction, network_connection_config)
            .await
    }
}
