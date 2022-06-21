#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct NetworkForTransactionArgs {
    ///What is the name of the network
    network_name: String,
    #[interactive_clap(named_arg)]
    transaction_signature_options: crate::transaction_signature_options::SignWithArgs,
}

impl NetworkForTransactionArgs {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        let connection_config: crate::common::ConnectionConfig = match self.network_name.as_str() {
            "testnet" => crate::common::ConnectionConfig::Testnet,
            "mainnet" => crate::common::ConnectionConfig::Mainnet,
            "betanet" => crate::common::ConnectionConfig::Betanet,
            _ => todo!(),
        };
        println!("===========================");
        println!(
            "===  prepopulated_unsigned_transaction: {:?}",
            &prepopulated_unsigned_transaction
        );
        println!("===========================");
        self.transaction_signature_options
            .process(prepopulated_unsigned_transaction, connection_config)
            .await
    }
}
