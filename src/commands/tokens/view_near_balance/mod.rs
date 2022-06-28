#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct ViewNearBalance {
    #[interactive_clap(named_arg)]
    /// Select online mode
    network: super::super::super::network_view_at_block::NetworkViewAtBlockArgs,
}

impl ViewNearBalance {
    pub async fn process(
        &self,
        owner_account_id: near_primitives::types::AccountId,
    ) -> crate::CliResult {
        let account_transfer_allowance = crate::common::get_account_transfer_allowance(
            &self.network.get_connection_config(),
            owner_account_id,
            self.network.get_block_ref(),
        )
        .await?;
        println! {"{}", &account_transfer_allowance};
        Ok(())
    }
}
