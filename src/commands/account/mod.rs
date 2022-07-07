use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod add_key;
mod create_subaccount;
mod delete_account;
mod delete_key;
mod list_keys;
mod view_account_summary;

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
    ///View properties for an account
    ViewAccountSummary(self::view_account_summary::ViewAccountSummary),
    #[strum_discriminants(strum(message = "Create a new sub-account"))]
    ///Create a new sub-account
    CreateSubaccount(self::create_subaccount::SubAccount),
    #[strum_discriminants(strum(message = "Delete this account"))]
    ///Delete this account
    DeleteAccount(self::delete_account::DeleteAccount),
    #[strum_discriminants(strum(message = "View a list of keys for an account"))]
    ///View a list of keys for an account
    ListKeys(self::list_keys::ViewListKeys),
    #[strum_discriminants(strum(message = "Add an access key for this account"))]
    ///Add an access key for this account
    AddKey(self::add_key::AddKeyCommand),
    DeleteKey,
    Login,
}

impl AccountActions {
    pub async fn process(&self) -> crate::CliResult {
        match self {
            Self::ViewAccountSummary(view_account_command) => view_account_command.process().await,
            Self::ListKeys(view_list_keys) => view_list_keys.process().await,
            Self::DeleteAccount(delete_account) => delete_account.process().await,
            Self::CreateSubaccount(sub_account) => sub_account.process().await,
            Self::AddKey(add_key_command) => add_key_command.process().await,
            _ => todo!(),
        }
    }
}
