#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct SendNearCommand {
    ///What is the receiver account ID?
    receiver_account_id: crate::types::account_id::AccountId,
    ///Enter an amount to transfer
    amount_in_near: crate::common::NearBalance,
    #[interactive_clap(named_arg)]
    ///Select online mode
    network: super::super::super::network_for_transaction::NetworkForTransactionArgs,
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
            receiver_id: self.receiver_account_id.clone().into(),
            ..prepopulated_unsigned_transaction
        };
        self.network.process(unsigned_transaction).await
    }
}
