use async_recursion::async_recursion;
use dialoguer::Input;

///Create transfer NEAR tokens
#[derive(Debug, Default, Clone, clap::Parser)]
#[clap(
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    // setting(clap::AppSettings::VersionlessSubcommands)
)]
pub struct CliSendNearCommand {
    ///Enter an amount to transfer
    amount_in_near: Option<crate::common::NearBalance>,
    #[clap(subcommand)]
    next_action: Option<super::CliSkipNextAction>,
}

#[derive(Debug, Clone)]
pub struct SendNearCommand {
    amount_in_near: crate::common::NearBalance,
    pub next_action: Box<super::NextAction>,
}

impl interactive_clap::ToCli for SendNearCommand {
    type CliVariant = CliSendNearCommand;
}

impl CliSendNearCommand {
    pub fn to_cli_args(&self) -> std::collections::VecDeque<String> {
        let mut args = self
            .next_action
            .as_ref()
            .map(|subcommand| subcommand.to_cli_args())
            .unwrap_or_default();
        if let Some(amount) = &self.amount_in_near {
            args.push_front(amount.to_string());
        }
        args
    }
}

impl From<SendNearCommand> for CliSendNearCommand {
    fn from(send_near_command: SendNearCommand) -> Self {
        Self {
            amount_in_near: Some(send_near_command.amount_in_near.into()),
            next_action: Some(super::CliSkipNextAction::Skip(super::CliSkipAction {
                network: None,
            })),
        }
    }
}

impl SendNearCommand {
    pub fn from_cli(
        optional_clap_variant: Option<<SendNearCommand as interactive_clap::ToCli>::CliVariant>,
        context: (),
    ) -> color_eyre::eyre::Result<Self> {
        let amount_in_near: crate::common::NearBalance = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.amount_in_near)
        {
            Some(cli_amount) => cli_amount,
            None => SendNearCommand::input_amount(&context)?,
        };

        let skip_next_action: super::NextAction =
            match optional_clap_variant.and_then(|clap_variant| clap_variant.next_action) {
                Some(cli_skip_action) => {
                    super::NextAction::from_cli_skip_next_action(cli_skip_action, context)?
                }
                None => super::NextAction::choose_variant(context)?,
            };

        Ok(Self {
            amount_in_near,
            next_action: Box::new(skip_next_action),
        })
    }
}

impl SendNearCommand {
    fn input_amount(_context: &()) -> color_eyre::eyre::Result<crate::common::NearBalance> {
        let input_amount: crate::common::NearBalance = Input::new()
                        .with_prompt("How many NEAR Tokens do you want to transfer? (example: 10NEAR or 0.5near or 10000yoctonear)")
                        .interact_text()
                        ?;
        Ok(input_amount)
    }

    #[async_recursion(?Send)]
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
            ..prepopulated_unsigned_transaction
        };
        match *self.next_action.clone() {
            super::NextAction::AddAction(select_action) => {
                select_action.process(unsigned_transaction).await
            }
            super::NextAction::Skip(skip_action) => skip_action.process(unsigned_transaction).await,
        }
    }
}
