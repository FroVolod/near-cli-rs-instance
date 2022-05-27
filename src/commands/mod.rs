use near_primitives::views::ViewApplyState;

mod tokens;

#[derive(clap::Subcommand, Debug, Clone)]
pub enum TopLevel {
    Account(AccountCommands),
    Contract(ContractCommands),
    Tokens(tokens::TokensCommands),
}

impl TopLevel {
    pub async fn process(&self) -> crate::CliResult {
        let unsigned_transaction = near_primitives::transaction::Transaction {
            signer_id: "test".parse().unwrap(),
            public_key: near_crypto::PublicKey::empty(near_crypto::KeyType::ED25519),
            nonce: 0,
            receiver_id: "test".parse().unwrap(),
            block_hash: Default::default(),
            actions: vec![],
        };
        match self {
            Self::Tokens(tokens_commands) => tokens_commands.process(unsigned_transaction).await,
            Self::Account(account_commands) => account_commands.process().await,
            _ => todo!(),
        }
    }
}

#[derive(clap::Args, Debug, Clone)]
pub struct AccountCommands {
    account_id: String,
    #[clap(subcommand)]
    network: Network<ViewAtBlock>,
}

impl AccountCommands {
    pub async fn process(
        &self,
    ) -> crate::CliResult {
        // TODO: use account_id
        self.network.process().await
    }
}

#[derive(clap::Args, Debug, Clone)]
pub struct ContractCommands {
    contract_id: String,
    #[clap(subcommand)]
    network: Network<crate::transaction_signature_options::SignWith>,
}

#[derive(clap::Subcommand, Debug, Clone)]
enum Network<Next: clap::Subcommand> {
    Network(NetworkArgs<Next>),
}

impl Network<crate::transaction_signature_options::SignWith> {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        match self {
            Self::Network(network) => network.process(prepopulated_unsigned_transaction).await,
        }
    }
}

impl Network<ViewAtBlock> {
    pub async fn process(
        &self,
    ) -> crate::CliResult {
        match self {
            Self::Network(network) => network.process().await,
        }
    }
}

#[derive(clap::Args, Debug, Clone)]
struct NetworkArgs<Next: clap::Subcommand> {
    network_name: String,
    #[clap(subcommand)]
    next: Next,
}

impl NetworkArgs<crate::transaction_signature_options::SignWith> {
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

        self.next.process(
            prepopulated_unsigned_transaction,
            connection_config,
        )
        .await
    }
}

impl NetworkArgs<ViewAtBlock> {
    async fn process(
        &self,
    ) -> crate::CliResult {
        let connection_config: crate::common::ConnectionConfig = match self.network_name.as_str() {
            "testnet" => crate::common::ConnectionConfig::Testnet,
            "mainnet" => crate::common::ConnectionConfig::Mainnet,
            "betanet" => crate::common::ConnectionConfig::Betanet,
            _ => todo!(),
        };

        self.next.process(
            connection_config,
        )
        .await
    }
}

#[derive(clap::Subcommand, Debug, Clone)]
enum ViewAtBlock {
    Now,
    AtBlockHeight(AtBlockHeight),
}

impl ViewAtBlock {
    async fn process(&self, connection_config: crate::common::ConnectionConfig) -> crate::CliResult {
        match self {
            _ => todo!("view at block process"),
        }
    }
}

#[derive(clap::Args, Debug, Clone)]
struct AtBlockHeight {
    block_height: u64,
}
