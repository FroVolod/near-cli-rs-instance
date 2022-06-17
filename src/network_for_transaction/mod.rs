#[derive(clap::Subcommand, Debug, Clone)]
pub enum NetworkForTransaction {
    Network(NetworkForTransactionArgs),
}

impl NetworkForTransaction {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        match self {
            Self::Network(network) => network.process(prepopulated_unsigned_transaction).await,
        }
    }
}

#[derive(clap::Args, Debug, Clone)]
pub struct NetworkForTransactionArgs {
    network_name: String,
    #[clap(subcommand)]
    transaction_signature_options: TransactionSignatureOptions,
}

impl NetworkForTransactionArgs {
    async fn process(
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

#[derive(clap::Subcommand, Debug, Clone)]
enum TransactionSignatureOptions {
    TransactionSignature(crate::transaction_signature_options::SignWithArgs),
}

impl TransactionSignatureOptions {
    async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        connection_config: crate::common::ConnectionConfig,
    ) -> crate::CliResult {
        match &self {
            Self::TransactionSignature(sign_with_args) => {
                sign_with_args
                    .process(prepopulated_unsigned_transaction, connection_config)
                    .await
            }
        }
    }
}
