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
        let view_item = crate::common::ViewItems::ViewNearBalance;
        self.network.process(owner_account_id, view_item).await
    }
}
