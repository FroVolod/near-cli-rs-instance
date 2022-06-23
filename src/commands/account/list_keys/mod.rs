#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct ViewListKeys {
    ///What Account ID do you need to view?
    account_id: crate::types::account_id::AccountId,
    #[interactive_clap(named_arg)]
    /// Select online mode
    network: super::super::super::network_view_at_block::NetworkViewAtBlockArgs,
}

impl ViewListKeys {
    pub async fn process(&self) -> crate::CliResult {
        let view_item = crate::common::ViewItems::ViewAccessKeyList;
        self.network
            .process(self.account_id.clone().into(), view_item)
            .await
    }
}
