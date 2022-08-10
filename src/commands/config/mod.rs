use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod add_connection;
mod delete_connection;

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
    // #[strum_discriminants(strum(
    //     message = "list-keys            - View a list of access keys of an account"
    // ))]
    // /// View a list of access keys of an account
    // List,
    #[strum_discriminants(strum(message = "add              - Add a network connection"))]
    ///Add an access key to an account
    Add(self::add_connection::AddNetworkConnection),
    #[strum_discriminants(strum(message = "delete           - Delete a network connection"))]
    ///Delete a network connection
    Delete(self::delete_connection::DeleteNetworkConnection),
}

impl ConfigActions {
    pub async fn process(&self, config: crate::config::Config) -> crate::CliResult {
        match self {
            Self::Add(add_network_connection) => add_network_connection.process(config).await,
            Self::Delete(delete_network_connection) => {
                delete_network_connection.process(config).await
            }
            _ => todo!(),
        }
    }
}
