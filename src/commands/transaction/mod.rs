use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod view_status;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct TransactionCommands {
    #[interactive_clap(subcommand)]
    transaction_actions: TransactionActions,
}

impl TransactionCommands {
    pub async fn process(&self) -> crate::CliResult {
        self.transaction_actions.process().await
    }
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Сhoose action for transaction
pub enum TransactionActions {
    #[strum_discriminants(strum(message = "View a transaction status"))]
    ///Execute function (contract method)
    ViewStatus(self::view_status::TransactionInfo),
    #[strum_discriminants(strum(message = "Construct a new transaction"))]
    ///Construct a new transaction
    ConstructTransaction,
}

impl TransactionActions {
    pub async fn process(&self) -> crate::CliResult {
        match self {
            Self::ViewStatus(transaction_info) => transaction_info.process().await,
            _ => todo!(),
        }
    }
}
