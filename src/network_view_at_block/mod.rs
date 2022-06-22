use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct NetworkViewAtBlockArgs {
    ///What is the name of the network
    network_name: String,
    #[interactive_clap(subcommand)]
    next: ViewAtBlock,
}

impl NetworkViewAtBlockArgs {
    pub async fn process(
        &self,
        account_id: near_primitives::types::AccountId,
        view_item: crate::common::ViewItems,
    ) -> crate::CliResult {
        let connection_config: crate::common::ConnectionConfig = match self.network_name.as_str() {
            "testnet" => crate::common::ConnectionConfig::Testnet,
            "mainnet" => crate::common::ConnectionConfig::Mainnet,
            "betanet" => crate::common::ConnectionConfig::Betanet,
            _ => todo!(),
        };

        self.next
            .process(account_id, connection_config, view_item)
            .await
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
        view_item: crate::common::ViewItems,
    ) -> crate::CliResult {
        match self {
            Self::Now => {
                match view_item {
                    crate::common::ViewItems::ViewAccessKeyList => {
                        crate::common::display_access_key_list(
                            account_id,
                            &connection_config,
                            near_primitives::types::Finality::Final.into(),
                        )
                        .await?;
                    }
                    crate::common::ViewItems::ViewAccountSummary => {
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
                    }
                    crate::common::ViewItems::ViewNonce => todo!("ViewNonce"),
                    crate::common::ViewItems::ViewCallFunction => todo!("ViewCallFunction"),
                    crate::common::ViewItems::ViewContractHash => todo!("ViewContractHash"),
                    crate::common::ViewItems::ViewContractCode => todo!("ViewContractCode"),
                    crate::common::ViewItems::ViewContractState => todo!("ViewContractState"),
                    crate::common::ViewItems::ViewTransactionStatus => {
                        todo!("ViewTransactionStatus")
                    }
                    crate::common::ViewItems::ViewNearBalance => {
                        let account_transfer_allowance =
                            crate::common::get_account_transfer_allowance(
                                &connection_config,
                                account_id,
                            )
                            .await?;
                        println! {"{}", &account_transfer_allowance};
                    }
                    crate::common::ViewItems::ViewFtBalance => todo!("ViewFtBalance"),
                    crate::common::ViewItems::ViewNftBalance => todo!("ViewNftBalance"),
                }

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
