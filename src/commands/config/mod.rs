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
/// What do you want to do with a near-cli config?
pub enum ConfigActions {
    #[strum_discriminants(strum(
        message = "list             - View a list of network connections"
    ))]
    /// View a list of list of network connections
    List,
    #[strum_discriminants(strum(message = "add              - Add a network connection"))]
    ///Add a network connection
    Add(self::add_connection::AddNetworkConnection),
    #[strum_discriminants(strum(message = "delete           - Delete a network connection"))]
    ///Delete a network connection
    Delete(self::delete_connection::DeleteNetworkConnection),
}

impl ConfigActions {
    pub async fn process(&self, config: crate::config::Config) -> crate::CliResult {
        match self {
            Self::List => {
                let config_toml = toml::to_string(&config)?;
                println!("\n{}", &config_toml);
                Ok(())
            }
            Self::Add(add_network_connection) => add_network_connection.process(config).await,
            Self::Delete(delete_network_connection) => {
                delete_network_connection.process(config).await
            }
            _ => todo!(),
        }
    }
}
