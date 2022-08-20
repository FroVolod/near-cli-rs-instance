use common::{try_external_subcommand_execution, CliResult};

mod commands;
mod common;
mod config;
mod network_for_transaction;
mod network_view_at_block;
mod transaction_signature_options;
mod types;
mod utils_command;

pub type GlobalContext = (crate::config::Config,);

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = crate::GlobalContext)]
struct Cmd {
    #[interactive_clap(subcommand)]
    top_level: self::commands::TopLevelCommand,
}

impl Cmd {
    async fn process(&self, config: crate::config::Config) -> CliResult {
        self.top_level.process(config).await
    }
}

fn main() -> CliResult {
    let config = crate::common::get_config_toml()?;

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

    let cmd = Cmd::from_cli(Some(cli), (config.clone(),))?;

    let completed_cli = CliCmd::from(cmd.clone());

    let process_result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(cmd.process(config));

    match process_result {
        Result::Ok(()) => {
            println!(
                "Your console command:\n./near-cli {}",
                shell_words::join(&completed_cli.to_cli_args())
            );
            Ok(())
        },
        Result::Err(err) => Result::Err(err)
    } 
}
