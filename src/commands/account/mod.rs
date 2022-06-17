mod view_account_summary;

#[derive(clap::Args, Debug, Clone)]
pub struct AccountCommands {
    #[clap(subcommand)]
    account_actions: AccountActions,
}

impl AccountCommands {
    pub async fn process(&self) -> crate::CliResult {
        self.account_actions.process().await
    }
}

#[derive(clap::Subcommand, Debug, Clone)]
enum AccountActions {
    ViewAccountSummary(self::view_account_summary::ViewAccountSummary),
    CreateSubaccount,
    DeleteAccount,
    ListKeys,
    AddKey,
    DeleteKey,
    Login,
}

impl AccountActions {
    pub async fn process(&self) -> crate::CliResult {
        match self {
            Self::ViewAccountSummary(view_account_command) => view_account_command.process().await,
            _ => todo!(),
        }
    }
}
