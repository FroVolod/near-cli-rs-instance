#[derive(clap::Args, Debug, Clone)]
pub struct TokensCommands {
    owner_account_id: near_primitives::types::AccountId,
    #[clap(subcommand)]
    tokens_actions: TokensActions,
}

impl TokensCommands {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        let unsigned_transaction = near_primitives::transaction::Transaction {
            signer_id: self.owner_account_id.clone(),
            ..prepopulated_unsigned_transaction
        };
        self.tokens_actions.process(unsigned_transaction).await
    }
}

#[derive(clap::Subcommand, Debug, Clone)]
enum TokensActions {
    SendNear(SendNearCommand),
    SendFt,
    SendNft,
    ViewNearBalance,
    ViewFtBalance,
    ViewNftBalance,
}

impl TokensActions {
    async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        match self {
            Self::SendNear(send_near_command) => {
                send_near_command
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
            _ => todo!(),
        }
    }
}

#[derive(clap::Args, Debug, Clone)]
struct SendNearCommand {
    receiver_account_id: near_primitives::types::AccountId,
    amount_in_near: crate::common::NearBalance,
    #[clap(subcommand)]
    network: super::NetworkArg<crate::transaction_signature_options::SignWith>,
}

impl SendNearCommand {
    async fn process(
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
