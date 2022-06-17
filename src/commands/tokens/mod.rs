mod send_near;

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
    SendNear(self::send_near::SendNearCommand),
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
