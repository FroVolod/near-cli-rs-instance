#[derive(clap::Subcommand, Debug, Clone)]
pub enum NetworkViewAtBlock {
    Network(NetworkViewAtBlockArgs),
}

impl NetworkViewAtBlock {
    pub async fn process(&self, account_id: near_primitives::types::AccountId) -> crate::CliResult {
        match self {
            Self::Network(network) => network.process(account_id).await,
        }
    }
}

#[derive(clap::Args, Debug, Clone)]
pub struct NetworkViewAtBlockArgs {
    network_name: String,
    #[clap(subcommand)]
    next: NextViewAtBlock,
}

impl NetworkViewAtBlockArgs {
    async fn process(&self, account_id: near_primitives::types::AccountId) -> crate::CliResult {
        let connection_config: crate::common::ConnectionConfig = match self.network_name.as_str() {
            "testnet" => crate::common::ConnectionConfig::Testnet,
            "mainnet" => crate::common::ConnectionConfig::Mainnet,
            "betanet" => crate::common::ConnectionConfig::Betanet,
            _ => todo!(),
        };

        self.next.process(account_id, connection_config).await
    }
}

#[derive(clap::Subcommand, Debug, Clone)]
enum NextViewAtBlock {
    ViewAtBlock(ViewAtBlockArgs),
}

impl NextViewAtBlock {
    async fn process(
        &self,
        account_id: near_primitives::types::AccountId,
        connection_config: crate::common::ConnectionConfig,
    ) -> crate::CliResult {
        match &self {
            Self::ViewAtBlock(view_at_block_args) => {
                view_at_block_args
                    .process(account_id, connection_config)
                    .await
            }
        }
    }
}

#[derive(clap::Args, Debug, Clone)]
struct ViewAtBlockArgs {
    #[clap(subcommand)]
    view_at_block: ViewAtBlock,
}

impl ViewAtBlockArgs {
    async fn process(
        &self,
        account_id: near_primitives::types::AccountId,
        connection_config: crate::common::ConnectionConfig,
    ) -> crate::CliResult {
        self.view_at_block
            .process(account_id, connection_config)
            .await
    }
}

#[derive(clap::Subcommand, Debug, Clone)]
enum ViewAtBlock {
    Now,
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

#[derive(clap::Args, Debug, Clone)]
struct AtBlockHeight {
    block_height: u64,
}
