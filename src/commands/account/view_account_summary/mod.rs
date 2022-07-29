#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = crate::GlobalContext)]
pub struct ViewAccountSummary {
    ///What Account ID do you need to view?
    account_id: crate::types::account_id::AccountId,
    #[interactive_clap(named_arg)]
    ///Select online mode
    network: crate::network_view_at_block::NetworkViewAtBlockArgs,
}

impl ViewAccountSummary {
    pub async fn process(&self, config: crate::config::Config) -> crate::CliResult {
        let connection_config = self.network.get_connection_config(config);
        crate::common::display_account_info(
            self.account_id.clone().into(),
            &connection_config,
            self.network.get_block_ref(),
        )
        .await?;
        crate::common::display_access_key_list(
            self.account_id.clone().into(),
            &connection_config,
            self.network.get_block_ref(),
        )
        .await?;
        Ok(())
    }
}
