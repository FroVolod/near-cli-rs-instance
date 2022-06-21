use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod account;
mod tokens;

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Choose action
pub enum TopLevelCommand {
    #[strum_discriminants(strum(
        message = "View account summary, create subaccount, delete account, list keys, add key, delete key"
    ))]
    ///View account summary, create subaccount, delete account, list keys, add key, delete key
    Account(self::account::AccountCommands),
    #[strum_discriminants(strum(message = "Use this for token actions"))]
    ///Use this for token actions
    Tokens(self::tokens::TokensCommands),
}

impl TopLevelCommand {
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

// #[derive(Debug, Clone, interactive_clap::InteractiveClap)]
// pub struct ContractCommands {
//     contract_id: String,
//     #[interactive_clap(subcommand)]
//     network: super::network_for_transaction::NetworkForTransaction,
// }
