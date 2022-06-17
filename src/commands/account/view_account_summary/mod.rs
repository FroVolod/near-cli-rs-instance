#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct ViewAccountSummary {
    ///What Account ID do you need to view?
    account_id: crate::types::account_id::AccountId,
    #[interactive_clap(named_arg)]
    /// Select online mode
    network: super::super::super::network_view_at_block::NetworkViewAtBlockArgs,
}

impl ViewAccountSummary {
    pub async fn process(&self) -> crate::CliResult {
        self.network.process(self.account_id.clone().into()).await
    }
}
