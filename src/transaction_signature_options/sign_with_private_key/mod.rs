use near_primitives::borsh::BorshSerialize;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = crate::GlobalContext)]
#[interactive_clap(skip_default_from_cli)]
pub struct SignPrivateKey {
    #[interactive_clap(long)]
    #[interactive_clap(skip_default_from_cli_arg)]
    #[interactive_clap(skip_default_input_arg)]
    pub signer_public_key: crate::types::public_key::PublicKey,
    #[interactive_clap(long)]
    #[interactive_clap(skip_default_from_cli_arg)]
    #[interactive_clap(skip_default_input_arg)]
    pub signer_private_key: crate::types::secret_key::SecretKey,
    #[interactive_clap(long)]
    #[interactive_clap(skip_default_from_cli_arg)]
    #[interactive_clap(skip_default_input_arg)]
    pub nonce: Option<u64>,
    #[interactive_clap(long)]
    #[interactive_clap(skip_default_from_cli_arg)]
    #[interactive_clap(skip_default_input_arg)]
    pub block_hash: Option<String>,
    #[interactive_clap(subcommand)]
    pub submit: super::Submit,
}

impl SignPrivateKey {
    pub fn from_cli(
        optional_clap_variant: Option<<SignPrivateKey as interactive_clap::ToCli>::CliVariant>,
        _context: crate::GlobalContext,
    ) -> color_eyre::eyre::Result<Self> {
        let signer_public_key: crate::types::public_key::PublicKey = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.signer_public_key)
        {
            Some(cli_public_key) => cli_public_key,
            None => super::input_signer_public_key()?,
        };
        let signer_private_key: crate::types::secret_key::SecretKey = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.signer_private_key)
        {
            Some(signer_private_key) => signer_private_key,
            None => super::input_signer_private_key()?,
        };
        let submit: super::Submit = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.submit)
        {
            Some(submit) => submit,
            None => super::Submit::choose_submit(),
        };
        Ok(Self {
            signer_public_key,
            signer_private_key,
            nonce: None,
            block_hash: None,
            submit,
        })
    }
}

impl SignPrivateKey {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: crate::common::ConnectionConfig,
    ) -> crate::CliResult {
        let signer_secret_key: near_crypto::SecretKey = self.signer_private_key.clone().into();
        let online_signer_access_key_response =
            near_jsonrpc_client::JsonRpcClient::connect(network_connection_config.rpc_url())
                .call(near_jsonrpc_client::methods::query::RpcQueryRequest {
                    block_reference: near_primitives::types::Finality::Final.into(),
                    request: near_primitives::views::QueryRequest::ViewAccessKey {
                        account_id: prepopulated_unsigned_transaction.signer_id.clone(),
                        public_key: self.signer_public_key.clone().into(),
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
            public_key: self.signer_public_key.clone().into(),
            block_hash: online_signer_access_key_response.block_hash,
            nonce: current_nonce + 1,
            ..prepopulated_unsigned_transaction
        };
        let signature = signer_secret_key.sign(unsigned_transaction.get_hash_and_size().0.as_ref());
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
