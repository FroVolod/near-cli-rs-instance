#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = crate::GlobalContext)]
pub struct NetworkForTransactionArgs {
    ///What is the name of the network
    network_name: String,
    #[interactive_clap(subcommand)]
    transaction_signature_options: crate::transaction_signature_options::SignWith,
}

impl NetworkForTransactionArgs {
    pub fn get_connection_config(
        &self,
        config: crate::config::Config,
    ) -> crate::common::ConnectionConfig {
        match self.network_name.as_str() {
            "testnet" => crate::common::ConnectionConfig::Testnet,
            "mainnet" => crate::common::ConnectionConfig::Mainnet,
            "betanet" => crate::common::ConnectionConfig::Betanet,
            _ => todo!(),
        }
    }

    pub fn get_sign_option(&self) -> crate::transaction_signature_options::SignWith {
        self.transaction_signature_options.clone()
    }
}
