use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod send_ft;
mod send_near;
mod view_near_balance;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct TokensCommands {
    ///What is your account ID?
    owner_account_id: crate::types::account_id::AccountId,
    #[interactive_clap(subcommand)]
    tokens_actions: TokensActions,
}

impl TokensCommands {
    pub async fn process(&self) -> crate::CliResult {
        self.tokens_actions
            .process(self.owner_account_id.clone().into())
            .await
    }
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Select actions with tokens
pub enum TokensActions {
    #[strum_discriminants(strum(message = "The transfer is carried out in NEAR tokens"))]
    ///The transfer is carried out in NEAR tokens
    SendNear(self::send_near::SendNearCommand),
    #[strum_discriminants(strum(message = "The transfer is carried out in FT tokens"))]
    ///The transfer is carried out in FT tokens
    SendFt(self::send_ft::SendFtCommand),
    SendNft,
    #[strum_discriminants(strum(message = "View the balance of Near tokens"))]
    ///View the balance of Near tokens
    ViewNearBalance(self::view_near_balance::ViewNearBalance),
    ViewFtBalance,
    ViewNftBalance,
}

impl TokensActions {
    async fn process(
        &self,
        owner_account_id: near_primitives::types::AccountId,
    ) -> crate::CliResult {
        match self {
            Self::SendNear(send_near_command) => send_near_command.process(owner_account_id).await,
            Self::ViewNearBalance(view_near_balance) => {
                view_near_balance.process(owner_account_id).await
            },
            Self::SendFt(send_ft_command) => send_ft_command.process(owner_account_id).await,
            _ => todo!(),
        }
    }
}
