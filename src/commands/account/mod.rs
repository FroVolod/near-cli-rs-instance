mod view_account_summary;
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct AccountCommands {
    #[interactive_clap(subcommand)]
    account_actions: AccountActions,
}

impl AccountCommands {
    pub async fn process(&self) -> crate::CliResult {
        self.account_actions.process().await
    }
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Сhoose action for account
pub enum AccountActions {
    #[strum_discriminants(strum(message = "View properties for an account"))]
    /// View properties for an account
    ViewAccountSummary(self::view_account_summary::ViewAccountSummary),
    CreateSubaccount,
    DeleteAccount,
    ListKeys,
    AddKey,
    DeleteKey,
    Login,
}

impl AccountActions {
    pub async fn process(&self) -> crate::CliResult {
        match self {
            Self::ViewAccountSummary(view_account_command) => view_account_command.process().await,
            _ => todo!(),
        }
    }
}
