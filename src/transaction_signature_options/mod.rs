mod sign_with_keychain;
pub mod sign_with_private_key;

#[derive(clap::Subcommand, Debug, Clone)]
pub enum SignWith {
    SignWithKeychain(sign_with_keychain::SignKeychain),
    SignWithLedger,
    SignWithPlaintextPrivateKey(sign_with_private_key::SignPrivateKey),
}

impl SignWith {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        connection_config: crate::common::ConnectionConfig,
    ) -> crate::CliResult {
        match self {
            Self::SignWithPlaintextPrivateKey(sign_private_key) => {
                sign_private_key
                    .process(prepopulated_unsigned_transaction, connection_config)
                    .await
            }
            _ => todo!(),
        }
    }
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Submit {
    Send,
    Display,
}

impl Submit {
    pub async fn process(
        &self,
        network_connection_config: crate::common::ConnectionConfig,
        signed_transaction: near_primitives::transaction::SignedTransaction,
        serialize_to_base64: String,
    ) -> crate::CliResult {
        match self {
            Submit::Send => {
                println!("Transaction sent ...");
                let json_rcp_client = near_jsonrpc_client::JsonRpcClient::connect(
                    network_connection_config.rpc_url().as_str(),
                );
                let transaction_info = loop {
                    let transaction_info_result = json_rcp_client
                        .call(near_jsonrpc_client::methods::broadcast_tx_commit::RpcBroadcastTxCommitRequest{signed_transaction: signed_transaction.clone()})
                        .await;
                    match transaction_info_result {
                        Ok(response) => {
                            break response;
                        }
                        Err(err) => match crate::common::rpc_transaction_error(err) {
                            Ok(_) => {
                                tokio::time::sleep(std::time::Duration::from_millis(100)).await
                            }
                            Err(report) => return color_eyre::eyre::Result::Err(report),
                        },
                    };
                };
                println!("{:#?}", transaction_info);
                Ok(())
            }
            Submit::Display => {
                println!("\nSerialize_to_base64:\n{}", &serialize_to_base64);
                Ok(())
            }
        }
    }
}
