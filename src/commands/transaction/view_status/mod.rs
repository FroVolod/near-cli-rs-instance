use std::str::FromStr;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = crate::GlobalContext)]
pub struct TransactionInfo {
    ///Enter the hash of the transaction you need to view
    transaction_hash: String,
    ///What is the signer account ID?
    signer_account_id: crate::types::account_id::AccountId,
    ///What is the name of the network
    network_name: String,
}

impl TransactionInfo {
    pub async fn process(&self, config: crate::config::Config) -> crate::CliResult {
        let connection_config = match self.network_name.as_str() {
            "testnet" => crate::common::ConnectionConfig::Testnet,
            "mainnet" => crate::common::ConnectionConfig::Mainnet,
            "betanet" => crate::common::ConnectionConfig::Betanet,
            _ => todo!(),
        };
        let query_view_transaction_status = near_jsonrpc_client::JsonRpcClient::connect(connection_config.rpc_url())
            .call(near_jsonrpc_client::methods::EXPERIMENTAL_tx_status::RpcTransactionStatusRequest {
                transaction_info: near_jsonrpc_client::methods::EXPERIMENTAL_tx_status::TransactionInfo::TransactionId {
                    hash: near_primitives::hash::CryptoHash::from_str(&self.transaction_hash).unwrap(),
                    account_id: self.signer_account_id.clone().into()
                }
            })
            .await
            .map_err(|err| {
                color_eyre::Report::msg(format!(
                    "Failed to fetch query for view transaction: {:?}",
                    err
                ))
            })?;
        println!("Transaction status: {:#?}", query_view_transaction_status);
        Ok(())
    }
}
