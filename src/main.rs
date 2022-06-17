use clap::Parser;
// use common::CliResult;
use common::{try_external_subcommand_execution, CliResult};

mod commands;
mod common;
mod consts;
mod network_for_transaction;
mod network_view_at_block;
mod transaction_signature_options;
mod types;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
struct Cmd {
    #[interactive_clap(subcommand)]
    top_level: self::commands::TopLevelCommand,
}

impl Cmd {
    async fn process(self) -> CliResult {
        self.top_level.process().await
    }
}

fn main() -> CliResult {
    color_eyre::install()?;

    let cli = match Cmd::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            if matches!(
                error.kind(),
                clap::error::ErrorKind::UnknownArgument | clap::error::ErrorKind::InvalidSubcommand
            ) {
                return try_external_subcommand_execution(error);
            }
            error.exit();
        }
    };

    // if let Some(self::commands::CliTopLevelCommand::GenerateShellCompletions(subcommand)) =
    //     cli.top_level_command
    // {
    //     subcommand.process();
    //     return Ok(());
    // }

    let cmd = Cmd::from_cli(Some(cli), ())?;

    let completed_cli = CliCmd::from(cmd.clone());

    let process_result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(cmd.process());

    println!(
        "Your console command:\n./near-cli {}",
        shell_words::join(&completed_cli.to_cli_args())
    );

    process_result
}
