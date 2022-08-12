use std::str::FromStr;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = crate::GlobalContext)]
pub struct TransactionInfo {
    ///Enter the hash of the transaction you need to view
    transaction_hash: String,
    ///What is the signer account ID?
    signer_account_id: crate::types::account_id::AccountId,
    ///What is the name of the network
    #[interactive_clap(skip_default_input_arg)]
    network_name: String,
}

impl TransactionInfo {
    fn input_network_name(context: &crate::GlobalContext) -> color_eyre::eyre::Result<String> {
        crate::common::input_network_name(context)
    }

    pub async fn process(&self, config: crate::config::Config) -> crate::CliResult {
        let networks = config.networks;
        let network_config = networks
            .get(self.network_name.as_str())
            .expect("Impossible to get network name!")
            .clone();
        let query_view_transaction_status = near_jsonrpc_client::JsonRpcClient::connect(network_config.rpc_url)
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
