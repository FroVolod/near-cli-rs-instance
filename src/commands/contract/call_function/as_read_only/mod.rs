#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct CallFunctionView {
    ///What is the account ID?
    account_id: crate::types::account_id::AccountId,
    ///What is the name of the function?
    function_name: String,
    ///Enter arguments to this function
    function_args: String,
    #[interactive_clap(named_arg)]
    ///Select online mode
    network: crate::network_view_at_block::NetworkViewAtBlockArgs,
}

impl CallFunctionView {
    pub async fn process(&self) -> crate::CliResult {
        let args: near_primitives::types::FunctionArgs =
            near_primitives::types::FunctionArgs::from(self.function_args.clone().into_bytes());
        let query_view_method_response = near_jsonrpc_client::JsonRpcClient::connect(
            self.network.get_connection_config().rpc_url(),
        )
        .call(near_jsonrpc_client::methods::query::RpcQueryRequest {
            block_reference: self.network.get_block_ref(),
            request: near_primitives::views::QueryRequest::CallFunction {
                account_id: self.account_id.clone().into(),
                method_name: self.function_name.clone(),
                args,
            },
        })
        .await
        .map_err(|err| {
            color_eyre::Report::msg(format!("Failed to fetch query for view method: {:?}", err))
        })?;
        let call_result =
            if let near_jsonrpc_primitives::types::query::QueryResponseKind::CallResult(result) =
                query_view_method_response.kind
            {
                result.result
            } else {
                return Err(color_eyre::Report::msg(format!("Error call result")));
            };

        let serde_call_result = if call_result.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::from_slice(&call_result)
                .map_err(|err| color_eyre::Report::msg(format!("serde json: {:?}", err)))?
        };
        println!("--------------");
        println!();
        println!("{}", serde_json::to_string_pretty(&serde_call_result)?);
        Ok(())
    }
}