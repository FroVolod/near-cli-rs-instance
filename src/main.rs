use clap::Parser;
use common::CliResult;
// use common::{try_external_subcommand_execution, CliResult};

mod commands;
mod common;
mod consts;
mod network_for_transaction;
mod network_view_at_block;
mod transaction_signature_options;
mod types;

#[derive(Parser, Debug, Clone)]
struct Cmd {
    #[clap(subcommand)]
    top_level: self::commands::TopLevel,
}

impl Cmd {
    async fn process(self) -> CliResult {
        self.top_level.process().await
    }
}

fn main() -> CliResult {
    let args = Cmd::parse();
    println!("{:#?}", args);

    let process_result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(args.process());

    // println!(
    //     "Your console command:\n{} {}",
    //     std::env::args().next().as_deref().unwrap_or("./near_cli"),
    //     shell_words::join(&completed_cli.to_cli_args())
    // );

    process_result
}
