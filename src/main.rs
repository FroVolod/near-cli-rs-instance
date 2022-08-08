use common::{try_external_subcommand_execution, CliResult};
use std::io::Write;

mod commands;
mod common;
mod config;
mod consts;
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
    async fn process(self, config: crate::config::Config) -> CliResult {
        self.top_level.process(config).await
    }
}

fn main() -> CliResult {
    let config_default = crate::config::Config::default();
    let config_default_toml = toml::to_string(&config_default)?;

    let home_dir = dirs::home_dir().expect("Impossible to get your home dir!");
    let mut path_config_toml = std::path::PathBuf::from(&home_dir);
    path_config_toml.push(".near-credentials");
    std::fs::create_dir_all(&path_config_toml)?;
    path_config_toml.push("config.toml");
    if !path_config_toml.is_file() {
        std::fs::File::create(&path_config_toml)
            .map_err(|err| color_eyre::Report::msg(format!("Failed to create file: {:?}", err)))?
            .write(config_default_toml.as_bytes())
            .map_err(|err| {
                color_eyre::Report::msg(format!("Failed to write to file: {:?}", err))
            })?;
        println!(
            "The data for the access key is saved in a file {}",
            &path_config_toml.display()
        );
    };
    let config_toml = std::fs::read_to_string(path_config_toml)?;
    let config: crate::config::Config = toml::from_str(&config_toml)?;

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

    println!(
        "Your console command:\n./near-cli {}",
        shell_words::join(&completed_cli.to_cli_args())
    );

    process_result
}
