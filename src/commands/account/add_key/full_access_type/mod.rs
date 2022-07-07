#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct FullAccessType {
    #[interactive_clap(subcommand)]
    pub access_key_mode: super::AccessKeyMode,
}

impl FullAccessType {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        self.access_key_mode
            .process(
                prepopulated_unsigned_transaction,
                near_primitives::account::AccessKeyPermission::FullAccess,
            )
            .await
    }
}
