use dialoguer::{theme::ColorfulTheme, Input, Select};
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

pub mod sign_with_keychain;
pub mod sign_with_ledger;
pub mod sign_with_private_key;

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Select a tool for signing the transaction
pub enum SignWith {
    #[strum_discriminants(strum(message = "Sign with keychain"))]
    ///Sign with keychain
    SignWithKeychain(self::sign_with_keychain::SignKeychain),
    #[strum_discriminants(strum(message = "Sign with ledger"))]
    ///Sign with ledger
    SignWithLedger(self::sign_with_ledger::SignLedger),
    #[strum_discriminants(strum(message = "Sign with private key"))]
    ///Sign with private key
    SignWithPlaintextPrivateKey(self::sign_with_private_key::SignPrivateKey),
}

pub fn input_signer_public_key() -> color_eyre::eyre::Result<crate::types::public_key::PublicKey> {
    Ok(Input::new()
        .with_prompt("Enter sender (signer) public key")
        .interact_text()?)
}

pub fn input_signer_private_key() -> color_eyre::eyre::Result<crate::types::secret_key::SecretKey> {
    Ok(Input::new()
        .with_prompt("Enter sender (signer) private (secret) key")
        .interact_text()?)
}

pub fn input_access_key_nonce(public_key: &str) -> color_eyre::eyre::Result<u64> {
    println!("Your public key: `{}`", public_key);
    Ok(Input::new()
        .with_prompt(
            "Enter transaction nonce for this public key (query the access key information with \
            `./near-cli view nonce \
                network testnet \
                account 'volodymyr.testnet' \
                public-key ed25519:...` incremented by 1)",
        )
        .interact_text()?)
}

pub fn input_block_hash() -> color_eyre::eyre::Result<crate::types::crypto_hash::CryptoHash> {
    let input_block_hash: crate::common::BlockHashAsBase58 = Input::new()
        .with_prompt(
            "Enter recent block hash (query information about the hash of the last block with \
            `./near-cli view recent-block-hash network testnet`)",
        )
        .interact_text()?;
    Ok(crate::types::crypto_hash::CryptoHash(
        input_block_hash.inner,
    ))
}

#[derive(Debug, EnumDiscriminants, Clone, clap::Parser, interactive_clap::ToCliArgs)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum Submit {
    #[strum_discriminants(strum(message = "I want to send the transaction to the network"))]
    Send,
    #[strum_discriminants(strum(
        message = "I only want to print base64-encoded transaction for JSON RPC input and exit"
    ))]
    Display,
}

impl interactive_clap::ToCli for Submit {
    type CliVariant = Submit;
}

impl Submit {
    pub fn choose_submit() -> Self {
        let variants = SubmitDiscriminants::iter().collect::<Vec<_>>();
        let submits = variants
            .iter()
            .map(|p| p.get_message().unwrap().to_owned())
            .collect::<Vec<_>>();
        let select_submit = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("How would you like to proceed")
            .items(&submits)
            .default(0)
            .interact()
            .unwrap();
        match variants[select_submit] {
            SubmitDiscriminants::Send => Submit::Send,
            SubmitDiscriminants::Display => Submit::Display,
        }
    }

    pub async fn process(
        &self,
        network_connection_config: crate::common::ConnectionConfig,
        signed_transaction: near_primitives::transaction::SignedTransaction,
        serialize_to_base64: String,
    ) -> crate::CliResult {
        match self {
            Submit::Send => {
                println!("Transaction sent ...");
                let json_rcp_client = near_jsonrpc_client::JsonRpcClient::connect(
                    network_connection_config.rpc_url().as_str(),
                );
                let transaction_info = loop {
                    let transaction_info_result = json_rcp_client
                        .call(near_jsonrpc_client::methods::broadcast_tx_commit::RpcBroadcastTxCommitRequest{signed_transaction: signed_transaction.clone()})
                        .await;
                    match transaction_info_result {
                        Ok(response) => {
                            break response;
                        }
                        Err(err) => match crate::common::rpc_transaction_error(err) {
                            Ok(_) => {
                                tokio::time::sleep(std::time::Duration::from_millis(100)).await
                            }
                            Err(report) => return color_eyre::eyre::Result::Err(report),
                        },
                    };
                };
                println!("{:#?}", transaction_info);
                Ok(())
            }
            Submit::Display => {
                println!("\nSerialize_to_base64:\n{}", &serialize_to_base64);
                Ok(())
            }
        }
    }
}
