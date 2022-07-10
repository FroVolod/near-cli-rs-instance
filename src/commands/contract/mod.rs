use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod call_function;
mod deploy;
mod download_wasm;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct ContractCommands {
    #[interactive_clap(subcommand)]
    contract_actions: ContractActions,
}

impl ContractCommands {
    pub async fn process(&self) -> crate::CliResult {
        self.contract_actions.process().await
    }
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Сhoose action for account
pub enum ContractActions {
    #[strum_discriminants(strum(message = "Execute function (contract method)"))]
    ///Execute function (contract method)
    CallFunction(self::call_function::CallFunctionCommands),
    #[strum_discriminants(strum(message = "Add a new contract code"))]
    ///Add a contract code
    Deploy(self::deploy::Contract),
    #[strum_discriminants(strum(message = "Download wasm"))]
    ///Download wasm
    DownloadWasm(self::download_wasm::DownloadContract),
    #[strum_discriminants(strum(message = "Inspect storage"))]
    ///Inspect storage
    InspectStorage,
}

impl ContractActions {
    pub async fn process(&self) -> crate::CliResult {
        match self {
            Self::CallFunction(call_function_commands) => call_function_commands.process().await,
            Self::Deploy(contract) => contract.process().await,
            Self::DownloadWasm(download_contract) => download_contract.process().await,
            _ => todo!(),
        }
    }
}
