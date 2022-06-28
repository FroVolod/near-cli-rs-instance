#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct ViewAccountSummary {
    ///What Account ID do you need to view?
    account_id: crate::types::account_id::AccountId,
    #[interactive_clap(named_arg)]
    ///Select online mode
    network: super::super::super::network_view_at_block::NetworkViewAtBlockArgs,
}

impl ViewAccountSummary {
    pub async fn process(&self) -> crate::CliResult {
        crate::common::display_account_info(
            self.account_id.clone().into(),
            &self.network.get_connection_config(),
            self.network.get_block_ref(),
        )
        .await?;
        crate::common::display_access_key_list(
            self.account_id.clone().into(),
            &self.network.get_connection_config(),
            self.network.get_block_ref(),
        )
        .await?;
        Ok(())
    }
}
