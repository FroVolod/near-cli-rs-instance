use dialoguer::Input;
use std::io::Write;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = crate::GlobalContext)]
pub struct DownloadContract {
    ///What is the contract account ID?
    account_id: crate::types::account_id::AccountId,
    #[interactive_clap(skip_default_input_arg)]
    ///Where to download the contract file?
    file_path: crate::types::path_buf::PathBuf,
    #[interactive_clap(named_arg)]
    ///Select online mode
    network: crate::network_view_at_block::NetworkViewAtBlockArgs,
}

impl DownloadContract {
    fn input_file_path(
        _context: &crate::GlobalContext,
    ) -> color_eyre::eyre::Result<crate::types::path_buf::PathBuf> {
        println!();
        let input_file_path: String = Input::new()
            .with_prompt("Where to download the contract file?")
            .with_initial_text(format!("**contract_account_id**.wasm"))
            .interact_text()?;
        let file_path = shellexpand::tilde(&input_file_path).as_ref().parse()?;
        Ok(file_path)
    }

    pub async fn process(&self, config: crate::config::Config) -> crate::CliResult {
        let query_view_method_response = near_jsonrpc_client::JsonRpcClient::connect(
            self.network.get_network_config(config).rpc_url.as_str(),
        )
        .call(near_jsonrpc_client::methods::query::RpcQueryRequest {
            block_reference: self.network.get_block_ref(),
            request: near_primitives::views::QueryRequest::ViewCode {
                account_id: self.account_id.clone().into(),
            },
        })
        .await
        .map_err(|err| {
            color_eyre::Report::msg(format!(
                "Failed to fetch query for view contract: {:?}",
                err
            ))
        })?;
        let call_access_view =
            if let near_jsonrpc_primitives::types::query::QueryResponseKind::ViewCode(result) =
                query_view_method_response.kind
            {
                result
            } else {
                return Err(color_eyre::Report::msg(format!("Error call result")));
            };
        let dir_name = self.file_path.0.parent().unwrap();
        std::fs::create_dir_all(&dir_name)?;
        std::fs::File::create(self.file_path.0.clone())
            .map_err(|err| color_eyre::Report::msg(format!("Failed to create file: {:?}", err)))?
            .write(&call_access_view.code)
            .map_err(|err| {
                color_eyre::Report::msg(format!("Failed to write to file: {:?}", err))
            })?;
        println!(
            "\nThe file {:?} was downloaded successfully",
            self.file_path.0
        );
        Ok(())
    }
}
