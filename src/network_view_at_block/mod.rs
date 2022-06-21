use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct NetworkViewAtBlockArgs {
    ///What is the name of the network
    network_name: String,
    #[interactive_clap(subcommand)]
    next: ViewAtBlock,
}

impl NetworkViewAtBlockArgs {
    pub async fn process(&self, account_id: near_primitives::types::AccountId) -> crate::CliResult {
        let connection_config: crate::common::ConnectionConfig = match self.network_name.as_str() {
            "testnet" => crate::common::ConnectionConfig::Testnet,
            "mainnet" => crate::common::ConnectionConfig::Mainnet,
            "betanet" => crate::common::ConnectionConfig::Betanet,
            _ => todo!(),
        };

        self.next.process(account_id, connection_config).await
    }
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Сhoose block for view
pub enum ViewAtBlock {
    #[strum_discriminants(strum(message = "View account properties in the final block"))]
    /// View account properties in the final block
    Now,
    #[strum_discriminants(strum(message = "View account properties in the selected block"))]
    /// View account properties in the selected block
    AtBlockHeight(AtBlockHeight),
}

impl ViewAtBlock {
    async fn process(
        &self,
        account_id: near_primitives::types::AccountId,
        connection_config: crate::common::ConnectionConfig,
    ) -> crate::CliResult {
        match self {
            Self::Now => {
                crate::common::display_account_info(
                    account_id.clone(),
                    &connection_config,
                    near_primitives::types::Finality::Final.into(),
                )
                .await?;
                crate::common::display_access_key_list(
                    account_id,
                    &connection_config,
                    near_primitives::types::Finality::Final.into(),
                )
                .await?;
                Ok(())
            }
            _ => Ok(println!("view at block process")),
        }
    }
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct AtBlockHeight {
    block_height: u64,
}
