use async_recursion::async_recursion;
use dialoguer::Input;
use serde_json::json;

///Create transfer NFT tokens
#[derive(Debug, Default, Clone, clap::Parser)]
#[clap(
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    // setting(clap::AppSettings::VersionlessSubcommands)
)]
pub struct CliSendNftCommand {
    ///What is the nft-contract account ID?
    nft_contract_account_id: Option<crate::types::account_id::AccountId>,
    token_id: Option<String>,
    #[clap(long = "prepaid-gas")]
    ///Enter gas for function call
    gas: Option<crate::common::NearGas>,
    #[clap(long = "attached-deposit")]
    ///Enter deposit for a function call
    deposit: Option<crate::common::NearBalance>,
    #[clap(subcommand)]
    next_action: Option<super::super::CliSkipNextAction>,
}

#[derive(Debug, Clone)]
pub struct SendNftCommand {
    nft_contract_account_id: crate::types::account_id::AccountId,
    token_id: String,
    gas: crate::common::NearGas,
    deposit: crate::common::NearBalance,
    pub next_action: Box<super::super::NextAction>,
}

impl interactive_clap::ToCli for SendNftCommand {
    type CliVariant = CliSendNftCommand;
}

impl CliSendNftCommand {
    pub fn to_cli_args(&self) -> std::collections::VecDeque<String> {
        let mut args = self
            .next_action
            .as_ref()
            .map(|subcommand| subcommand.to_cli_args())
            .unwrap_or_default();
        if let Some(deposit) = &self.deposit {
            args.push_front(deposit.to_string());
            args.push_front("--attached-deposit".to_owned())
        }
        if let Some(gas) = &self.gas {
            args.push_front(gas.to_string());
            args.push_front("--prepaid-gas".to_owned());
        }
        if let Some(token_id) = &self.token_id {
            args.push_front(token_id.to_string());
        }
        if let Some(nft_contract_account_id) = &self.nft_contract_account_id {
            args.push_front(nft_contract_account_id.to_string());
        }
        args
    }
}

impl From<SendNftCommand> for CliSendNftCommand {
    fn from(send_near_command: SendNftCommand) -> Self {
        Self {
            nft_contract_account_id: Some(send_near_command.nft_contract_account_id),
            token_id: Some(send_near_command.token_id),
            gas: Some(send_near_command.gas),
            deposit: Some(send_near_command.deposit),
            next_action: Some(super::super::CliSkipNextAction::Skip(
                super::super::CliSkipAction { network: None },
            )),
        }
    }
}

impl SendNftCommand {
    pub fn from_cli(
        optional_clap_variant: Option<<SendNftCommand as interactive_clap::ToCli>::CliVariant>,
        context: (),
    ) -> color_eyre::eyre::Result<Self> {
        let nft_contract_account_id: crate::types::account_id::AccountId =
            match optional_clap_variant
                .clone()
                .and_then(|clap_variant| clap_variant.nft_contract_account_id)
            {
                Some(nft_contract_account_id) => nft_contract_account_id,
                None => SendNftCommand::input_nft_contract_account_id(&context)?,
            };
        let token_id: String = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.token_id)
        {
            Some(token_id) => token_id,
            None => SendNftCommand::input_token_id(&context)?,
        };
        let gas: crate::common::NearGas = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.gas)
        {
            Some(gas) => gas,
            None => SendNftCommand::input_gas(&context)?,
        };
        let deposit: crate::common::NearBalance = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.deposit)
        {
            Some(deposit) => deposit,
            None => SendNftCommand::input_deposit(&context)?,
        };

        let skip_next_action: super::super::NextAction =
            match optional_clap_variant.and_then(|clap_variant| clap_variant.next_action) {
                Some(cli_skip_action) => {
                    super::super::NextAction::from_cli_skip_next_action(cli_skip_action, context)?
                }
                None => super::super::NextAction::choose_variant(context)?,
            };

        Ok(Self {
            nft_contract_account_id,
            token_id,
            gas,
            deposit,
            next_action: Box::new(skip_next_action),
        })
    }
}

impl SendNftCommand {
    fn input_nft_contract_account_id(
        _context: &(),
    ) -> color_eyre::eyre::Result<crate::types::account_id::AccountId> {
        let nft_contract_account_id: crate::types::account_id::AccountId = Input::new()
            .with_prompt("What is the nft-contract account ID?")
            .interact_text()?;
        Ok(nft_contract_account_id)
    }

    fn input_token_id(_context: &()) -> color_eyre::eyre::Result<String> {
        println!();
        let token_id: String = Input::new()
            .with_prompt("Enter an amount NFT to transfer.")
            .interact_text()?;
        Ok(token_id)
    }

    fn input_gas(_context: &()) -> color_eyre::eyre::Result<crate::common::NearGas> {
        println!();
        let gas: u64 = loop {
            let input_gas: crate::common::NearGas = Input::new()
                .with_prompt("Enter gas for function call")
                .with_initial_text("100 TeraGas")
                .interact_text()?;
            let gas: u64 = match input_gas {
                crate::common::NearGas { inner: num } => num,
            };
            if gas <= 300000000000000 {
                break gas;
            } else {
                println!("You need to enter a value of no more than 300 TERAGAS")
            }
        };
        Ok(gas.into())
    }

    fn input_deposit(_context: &()) -> color_eyre::eyre::Result<crate::common::NearBalance> {
        println!();
        let deposit: crate::common::NearBalance = Input::new()
            .with_prompt(
                "Enter deposit for a function call (example: 10NEAR or 0.5near or 10000yoctonear).",
            )
            .with_initial_text("1 yoctoNEAR")
            .interact_text()?;
        Ok(deposit)
    }

    #[async_recursion(?Send)]
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        let method_name = "nft_transfer".to_string();
        let args = json!({
            "receiver_id": prepopulated_unsigned_transaction.receiver_id.to_string(),
            "token_id": self.token_id
        })
        .to_string()
        .into_bytes();
        let action = near_primitives::transaction::Action::FunctionCall(
            near_primitives::transaction::FunctionCallAction {
                method_name,
                args,
                gas: self.gas.clone().inner,
                deposit: self.deposit.clone().to_yoctonear(),
            },
        );
        let mut actions = prepopulated_unsigned_transaction.actions.clone();
        actions.push(action);
        let unsigned_transaction = near_primitives::transaction::Transaction {
            receiver_id: self.nft_contract_account_id.clone().into(),
            actions,
            ..prepopulated_unsigned_transaction
        };
        match *self.next_action.clone() {
            super::super::NextAction::AddAction(select_action) => {
                select_action.process(unsigned_transaction).await
            }
            super::super::NextAction::Skip(skip_action) => {
                skip_action.process(unsigned_transaction).await
            }
        }
    }
}
