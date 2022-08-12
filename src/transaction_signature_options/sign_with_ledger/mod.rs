use dialoguer::Input;
use near_primitives::borsh::BorshSerialize;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = crate::GlobalContext)]
#[interactive_clap(skip_default_from_cli)]
pub struct SignLedger {
    #[interactive_clap(long)]
    #[interactive_clap(skip_default_from_cli_arg)]
    #[interactive_clap(skip_default_input_arg)]
    seed_phrase_hd_path: crate::types::slip10::BIP32Path,
    #[interactive_clap(skip)]
    signer_public_key: crate::types::public_key::PublicKey,
    #[interactive_clap(long)]
    #[interactive_clap(skip_default_from_cli_arg)]
    #[interactive_clap(skip_default_input_arg)]
    nonce: Option<u64>,
    #[interactive_clap(long)]
    #[interactive_clap(skip_default_from_cli_arg)]
    #[interactive_clap(skip_default_input_arg)]
    block_hash: Option<String>,
    #[interactive_clap(subcommand)]
    submit: super::Submit,
}

impl SignLedger {
    pub fn from_cli(
        optional_clap_variant: Option<<SignLedger as interactive_clap::ToCli>::CliVariant>,
        _context: crate::GlobalContext,
    ) -> color_eyre::eyre::Result<Self> {
        let seed_phrase_hd_path = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.seed_phrase_hd_path)
        {
            Some(hd_path) => hd_path,
            None => SignLedger::input_seed_phrase_hd_path(),
        };
        println!(
            "Please allow getting the PublicKey on Ledger device (HD Path: {})",
            seed_phrase_hd_path
        );
        let public_key = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                near_ledger::get_public_key(seed_phrase_hd_path.clone().into()).await
            })
            .map_err(|near_ledger_error| {
                color_eyre::Report::msg(format!(
                    "An error occurred while trying to get PublicKey from Ledger device: {:?}",
                    near_ledger_error
                ))
            })?;
        let signer_public_key: crate::types::public_key::PublicKey =
            near_crypto::PublicKey::ED25519(near_crypto::ED25519PublicKey::from(
                public_key.to_bytes(),
            ))
            .into();
        let submit: super::Submit = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.submit)
        {
            Some(submit) => submit,
            None => super::Submit::choose_submit(),
        };
        Ok(Self {
            seed_phrase_hd_path,
            signer_public_key,
            nonce: None,
            block_hash: None,
            submit,
        })
    }

    pub fn input_seed_phrase_hd_path() -> crate::types::slip10::BIP32Path {
        Input::new()
            .with_prompt("Enter seed phrase HD Path (if you not sure leave blank for default)")
            .with_initial_text("44'/397'/0'/0'/1'")
            .interact_text()
            .unwrap()
    }

    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: crate::common::ConnectionConfig,
        network_config: crate::config::NetworkConfig,
    ) -> crate::CliResult {
        let seed_phrase_hd_path = self.seed_phrase_hd_path.clone().into();
        let online_signer_access_key_response =
            near_jsonrpc_client::JsonRpcClient::connect(network_config.rpc_url.clone())
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
        println!("\nUnsigned transaction:\n");
        crate::common::print_transaction(unsigned_transaction.clone());
        println!(
            "Confirm transaction signing on your Ledger device (HD Path: {})",
            seed_phrase_hd_path,
        );
        let signature = match near_ledger::sign_transaction(
            unsigned_transaction
                .try_to_vec()
                .expect("Transaction is not expected to fail on serialization"),
            seed_phrase_hd_path,
        )
        .await
        {
            Ok(signature) => {
                near_crypto::Signature::from_parts(near_crypto::KeyType::ED25519, &signature)
                    .expect("Signature is not expected to fail on deserialization")
            }
            Err(near_ledger_error) => {
                return Err(color_eyre::Report::msg(format!(
                    "Error occurred while signing the transaction: {:?}",
                    near_ledger_error
                )));
            }
        };

        let signed_transaction =
            near_primitives::transaction::SignedTransaction::new(signature, unsigned_transaction);
        let serialize_to_base64 = near_primitives::serialize::to_base64(
            signed_transaction
                .try_to_vec()
                .expect("Transaction is not expected to fail on serialization"),
        );
        println!("Your transaction was signed successfully.");
        self.submit
            .process(
                network_connection_config,
                network_config,
                signed_transaction,
                serialize_to_base64,
            )
            .await
    }
}
