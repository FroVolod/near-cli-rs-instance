use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod delete_account;
mod transfer_tokens;

#[derive(Debug, Clone, clap::Parser)]
pub enum CliSkipNextAction {
    /// Go to transaction signing
    Skip(CliSkipAction),
}

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Select an action that you want to add to the action:
pub enum NextAction {
    #[strum_discriminants(strum(message = "Select a new action"))]
    /// Choose next action
    AddAction(SelectAction),
    #[strum_discriminants(strum(message = "Skip adding a new action"))]
    /// Go to transaction signing
    Skip(SkipAction),
}

impl CliSkipNextAction {
    pub fn to_cli_args(&self) -> std::collections::VecDeque<String> {
        match self {
            Self::Skip(subcommand) => {
                // let mut args = ;  it is not implemented now!!!
                // args.push_front("skip".to_owned());
                // args
                subcommand.to_cli_args()
            }
        }
    }
}

impl From<NextAction> for CliSkipNextAction {
    fn from(next_action: NextAction) -> Self {
        match next_action {
            NextAction::AddAction(_select_action) => Self::Skip(CliSkipAction { network: None }),
            NextAction::Skip(skip_action) => Self::Skip(skip_action.into()),
        }
    }
}

impl NextAction {
    pub fn from_cli_skip_next_action(
        item: CliSkipNextAction,
        context: (),
    ) -> color_eyre::eyre::Result<Self> {
        match item {
            CliSkipNextAction::Skip(cli_skip_action) => {
                let skip_action: SkipAction = SkipAction::from_cli(Some(cli_skip_action), context)?;
                Ok(Self::Skip(skip_action))
            }
        }
    }
}
//-------------------------------------
/// This mode is not provided now
// impl CliNextAction {
//     fn from(item: CliSkipNextAction) -> color_eyre::eyre::Result<Self> {
//         match item {
//             CliSkipNextAction::Skip(cli_skip_action) => Ok(Self::Skip(cli_skip_action)),
//         }
//     }
// }
//--------------------------------------
impl NextAction {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        match self {
            NextAction::AddAction(select_action) => {
                select_action
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
            NextAction::Skip(skip_action) => {
                skip_action.process(prepopulated_unsigned_transaction).await
            }
        }
    }
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct SelectAction {
    #[interactive_clap(subcommand)]
    transaction_subcommand: ActionSubcommand,
}

impl SelectAction {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        self.transaction_subcommand
            .process(prepopulated_unsigned_transaction)
            .await
    }
}

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Select an action that you want to add to the action:
pub enum ActionSubcommand {
    #[strum_discriminants(strum(message = "Transfer tokens"))]
    ///Specify data for transfer tokens
    TransferTokens(self::transfer_tokens::SendNearCommand),
    #[strum_discriminants(strum(message = "Call the function"))]
    ///Specify data to call the function
    CallFunction, //(self::call_function_type::CallFunctionAction),
    #[strum_discriminants(strum(message = "Stake NEAR Tokens"))]
    ///Specify data to stake NEAR Tokens
    StakeNearTokens, //(self::stake_near_tokens_type::StakeNEARTokensAction),
    #[strum_discriminants(strum(message = "Create a sub-account"))]
    ///Specify data to create a sub-account
    CreateSubAccount, //(self::create_account_type::CreateAccountAction),
    #[strum_discriminants(strum(message = "Delete an account"))]
    ///Specify data to delete an account
    DeleteAccount(self::delete_account::DeleteAccountAction),
    #[strum_discriminants(strum(message = "Add an access key to the account"))]
    ///Specify the data to add an access key to the account
    AddAccessKey, //(self::add_access_key_mode::AddAccessKeyMode),
    #[strum_discriminants(strum(message = "Delete the access key to the account"))]
    ///Specify the data to delete the access key to the account
    DeleteAccessKey, //(self::delete_access_key_type::DeleteAccessKeyAction),
    #[strum_discriminants(strum(message = "Deploy the contract code"))]
    ///Specify the details to deploy the contract code
    DeployContract, //(self::add_contract_code_type::ContractFile),
}

impl ActionSubcommand {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        // network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> crate::CliResult {
        match self {
            ActionSubcommand::TransferTokens(args_transfer) => {
                args_transfer
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
            // ActionSubcommand::CallFunction(args_function) => {
            //     args_function
            //         .process(prepopulated_unsigned_transaction, network_connection_config)
            //         .await
            // }
            // ActionSubcommand::StakeNearTokens(args_stake) => {
            //     args_stake
            //         .process(prepopulated_unsigned_transaction, network_connection_config)
            //         .await
            // }
            // ActionSubcommand::CreateAccount(args_create_account) => {
            //     args_create_account
            //         .process(prepopulated_unsigned_transaction, network_connection_config)
            //         .await
            // }
            ActionSubcommand::DeleteAccount(args_delete_account) => {
                args_delete_account
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
            // ActionSubcommand::AddAccessKey(args_add_access_key) => {
            //     args_add_access_key
            //         .process(prepopulated_unsigned_transaction, network_connection_config)
            //         .await
            // }
            // ActionSubcommand::DeleteAccessKey(args_delete_access_key) => {
            //     args_delete_access_key
            //         .process(prepopulated_unsigned_transaction, network_connection_config)
            //         .await
            // }
            // ActionSubcommand::AddContractCode(args_contract_file) => {
            //     args_contract_file
            //         .process(prepopulated_unsigned_transaction, network_connection_config)
            //         .await
            // }
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct SkipAction {
    #[interactive_clap(named_arg)]
    ///Select online mode
    network: crate::network_for_transaction::NetworkForTransactionArgs,
}
//------------------------------------
// impl From<SelectAction> for CliSkipAction {
//     fn from(select_action: SelectAction) -> Self {
//         Self{
//             sign_option:
//         }
//     }
// }
//-----------------------------------------

impl SkipAction {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        match self.network.get_sign_option() {
            crate::transaction_signature_options::SignWith::SignWithPlaintextPrivateKey(
                sign_private_key,
            ) => {
                sign_private_key
                    .process(
                        prepopulated_unsigned_transaction,
                        self.network.get_connection_config(),
                    )
                    .await
            }
            crate::transaction_signature_options::SignWith::SignWithKeychain(sign_keychain) => {
                sign_keychain
                    .process(
                        prepopulated_unsigned_transaction,
                        self.network.get_connection_config(),
                    )
                    .await
            }
            crate::transaction_signature_options::SignWith::SignWithLedger(sign_ledger) => {
                sign_ledger
                    .process(
                        prepopulated_unsigned_transaction,
                        self.network.get_connection_config(),
                    )
                    .await
            }
        }
    }
}
