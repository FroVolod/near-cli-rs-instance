use near_primitives::borsh::BorshSerialize;

#[derive(clap::Args, Debug, Clone)]
pub struct SignPrivateKey {
    #[clap(long)]
    pub signer_public_key: near_crypto::PublicKey,
    #[clap(long)]
    pub signer_private_key: near_crypto::SecretKey,
    #[clap(long)]
    pub nonce: Option<u64>,
    #[clap(long)]
    pub block_hash: Option<String>,
    #[clap(subcommand)]
    pub submit: super::Submit,
}

impl SignPrivateKey {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: crate::common::ConnectionConfig,
    ) -> crate::CliResult {
        let online_signer_access_key_response = near_jsonrpc_client::JsonRpcClient::connect(
            &network_connection_config.rpc_url().as_str(),
        )
        .call(near_jsonrpc_client::methods::query::RpcQueryRequest {
            block_reference: near_primitives::types::Finality::Final.into(),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id: prepopulated_unsigned_transaction.signer_id.clone(),
                public_key: self.signer_public_key.clone(),
            },
        })
        .await
        .map_err(|err| {
            color_eyre::Report::msg(format!(
                "Failed to fetch public key information for nonce: {:?}",
                err
            ))
        })?;
        let current_nonce =
            if let near_jsonrpc_primitives::types::query::QueryResponseKind::AccessKey(
                online_signer_access_key,
            ) = online_signer_access_key_response.kind
            {
                online_signer_access_key.nonce
            } else {
                return Err(color_eyre::Report::msg(format!("Error current_nonce")));
            };
        let unsigned_transaction = near_primitives::transaction::Transaction {
            public_key: self.signer_public_key.clone(),
            block_hash: online_signer_access_key_response.block_hash,
            nonce: current_nonce + 1,
            ..prepopulated_unsigned_transaction
        };
        let signature = self
            .signer_private_key
            .sign(unsigned_transaction.get_hash_and_size().0.as_ref());
        let signed_transaction =
            near_primitives::transaction::SignedTransaction::new(signature, unsigned_transaction);
        let serialize_to_base64 = near_primitives::serialize::to_base64(
            signed_transaction
                .try_to_vec()
                .expect("Transaction is not expected to fail on serialization"),
        );
        println!("\nSigned transaction:\n");
        crate::common::print_transaction(signed_transaction.transaction.clone());
        println!("Your transaction was signed successfully.");
        self.submit
            .process(
                network_connection_config,
                signed_transaction,
                serialize_to_base64,
            )
            .await
    }
}
