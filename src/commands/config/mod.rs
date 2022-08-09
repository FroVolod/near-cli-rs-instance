use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod add_connection;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = crate::GlobalContext)]
pub struct ConfigCommands {
    #[interactive_clap(subcommand)]
    config_actions: ConfigActions,
}

impl ConfigCommands {
    pub async fn process(&self, config: crate::config::Config) -> crate::CliResult {
        self.config_actions.process(config).await
    }
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = crate::GlobalContext)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
/// What do you want to do with an account?
pub enum ConfigActions {
    #[strum_discriminants(strum(
        message = "view-account-summary - View properties for an account"
    ))]
    /// View properties for an account
    ViewAccountSummary,
    // #[strum_discriminants(strum(
    //     message = "login                - Log in with NEAR Wallet authorization"
    // ))]
    // /// Log in with NEAR Wallet
    // ImportAccount(self::import_account::Login),
    // #[strum_discriminants(strum(message = "create-subaccount    - Create a new sub-account"))]
    // /// Create a new sub-account
    // CreateSubaccount(self::create_subaccount::SubAccount),
    // #[strum_discriminants(strum(message = "delete-account       - Delete an account"))]
    // /// Delete an account
    // DeleteAccount(self::delete_account::DeleteAccount),
    #[strum_discriminants(strum(
        message = "list-keys            - View a list of access keys of an account"
    ))]
    /// View a list of access keys of an account
    List,
    #[strum_discriminants(strum(message = "add              - Add a network connection"))]
    /// Add an access key to an account
    Add(self::add_connection::AddNetworkConnection),
    // #[strum_discriminants(strum(
    //     message = "delete-key           - Delete an access key from an account"
    // ))]
    // /// Delete an access key from an account
    // DeleteKey(self::delete_key::DeleteKeyCommand),
}

impl ConfigActions {
    pub async fn process(&self, config: crate::config::Config) -> crate::CliResult {
        match self {
            // Self::ViewAccountSummary(view_account_command) => {
            //     view_account_command.process(config).await
            // }
            // Self::ListKeys(view_list_keys) => view_list_keys.process(config).await,
            // Self::DeleteAccount(delete_account) => delete_account.process(config).await,
            // Self::CreateSubaccount(sub_account) => sub_account.process(config).await,
            Self::Add(add_network_connection) => add_network_connection.process(config).await,
            // Self::DeleteKey(delete_key_command) => delete_key_command.process(config).await,
            // Self::ImportAccount(login) => login.process(config).await,
            _ => todo!(),
        }
    }
}
