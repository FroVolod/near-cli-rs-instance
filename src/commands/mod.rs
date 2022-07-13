use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod account;
mod contract;
mod tokens;
mod transaction;

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Choose action
pub enum TopLevelCommand {
    #[strum_discriminants(strum(
        message = "View account summary, create subaccount, delete account, list keys, add key, delete key, import account"
    ))]
    ///View account summary, create subaccount, delete account, list keys, add key, delete key, import account
    Account(self::account::AccountCommands),
    #[strum_discriminants(strum(
        message = "Use this for token actions: send near, send ft, send nft, view near balance, view ft balance, view nft balance"
    ))]
    ///Use this for token actions: send near, send ft, send nft, view near balance, view ft balance, view nft balance
    Tokens(self::tokens::TokensCommands),
    #[strum_discriminants(strum(
        message = "Use this for contract actions: call function, deploy, download wasm, inspect storage"
    ))]
    ///Use this for contract actions: call function, deploy, download wasm, inspect storage
    Contract(self::contract::ContractCommands),
    #[strum_discriminants(strum(
        message = "Use this to construct transactions or view the status of a transaction."
    ))]
    ///Use this to construct transactions or view the status of a transaction.
    Transaction(self::transaction::TransactionCommands),
}

impl TopLevelCommand {
    pub async fn process(&self) -> crate::CliResult {
        match self {
            Self::Tokens(tokens_commands) => tokens_commands.process().await,
            Self::Account(account_commands) => account_commands.process().await,
            Self::Contract(contract_commands) => contract_commands.process().await,
            Self::Transaction(transaction_commands) => transaction_commands.process().await,
        }
    }
}
