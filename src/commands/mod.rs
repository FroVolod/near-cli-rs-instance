use near_primitives::views::ViewApplyState;

mod account;
mod tokens;

#[derive(clap::Subcommand, Debug, Clone)]
pub enum TopLevel {
    Account(self::account::AccountCommands),
    // Contract(ContractCommands),
    Tokens(self::tokens::TokensCommands),
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
pub struct ContractCommands {
    contract_id: String,
    #[clap(subcommand)]
    network: super::network_for_transaction::NetworkForTransaction,
}
