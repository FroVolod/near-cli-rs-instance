#[derive(clap::Args, Debug, Clone)]
pub struct SendNearCommand {
    receiver_account_id: near_primitives::types::AccountId,
    amount_in_near: crate::common::NearBalance,
    #[clap(subcommand)]
    network: super::super::super::network_for_transaction::NetworkForTransaction,
}

impl SendNearCommand {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        let action = near_primitives::transaction::Action::Transfer(
            near_primitives::transaction::TransferAction {
                deposit: self.amount_in_near.to_yoctonear(),
            },
        );
        let mut actions = prepopulated_unsigned_transaction.actions.clone();
        actions.push(action);
        let unsigned_transaction = near_primitives::transaction::Transaction {
            actions,
            receiver_id: self.receiver_account_id.clone(),
            ..prepopulated_unsigned_transaction
        };
        self.network.process(unsigned_transaction).await
    }
}
