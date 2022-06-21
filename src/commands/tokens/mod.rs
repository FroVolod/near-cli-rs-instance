mod send_near;
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct TokensCommands {
    ///What is your account ID?
    owner_account_id: crate::types::account_id::AccountId,
    #[interactive_clap(subcommand)]
    tokens_actions: TokensActions,
}

impl TokensCommands {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        let unsigned_transaction = near_primitives::transaction::Transaction {
            signer_id: self.owner_account_id.clone().into(),
            ..prepopulated_unsigned_transaction
        };
        self.tokens_actions.process(unsigned_transaction).await
    }
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Select actions with tokens
pub enum TokensActions {
    #[strum_discriminants(strum(message = "The transfer is carried out in NEAR tokens"))]
    ///The transfer is carried out in NEAR tokens
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
