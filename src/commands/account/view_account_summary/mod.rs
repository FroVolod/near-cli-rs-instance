#[derive(clap::Args, Debug, Clone)]
pub struct ViewAccountSummary {
    account_id: near_primitives::types::AccountId,
    #[clap(subcommand)]
    network: super::super::super::network_view_at_block::NetworkViewAtBlock,
}

impl ViewAccountSummary {
    pub async fn process(&self) -> crate::CliResult {
        self.network.process(self.account_id.clone()).await
    }
}
