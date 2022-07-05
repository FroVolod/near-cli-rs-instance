use std::str::FromStr;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
pub struct GenerateKeypair {
    #[interactive_clap(named_arg)]
    ///Select online mode
    network: crate::network_for_transaction::NetworkForTransactionArgs,
}

impl GenerateKeypair {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        let key_pair_properties: crate::common::KeyPairProperties =
            crate::common::generate_keypair().await?;
        crate::common::save_access_key_to_keychain(
            Some(self.network.get_connection_config()),
            key_pair_properties.clone(),
            &prepopulated_unsigned_transaction.receiver_id.to_string(),
        )
        .await
        .map_err(|err| {
            color_eyre::Report::msg(format!("Failed to save a file with access key: {}", err))
        })?;

        let access_key: near_primitives::account::AccessKey = near_primitives::account::AccessKey {
            nonce: 0,
            permission: near_primitives::account::AccessKeyPermission::FullAccess,
        };
        let action = near_primitives::transaction::Action::AddKey(
            near_primitives::transaction::AddKeyAction {
                public_key: near_crypto::PublicKey::from_str(&key_pair_properties.public_key_str)?,
                access_key,
            },
        );
        let mut actions = prepopulated_unsigned_transaction.actions.clone();
        actions.push(action);
        let prepopulated_unsigned_transaction = near_primitives::transaction::Transaction {
            actions,
            ..prepopulated_unsigned_transaction
        };
        match self.network.get_sign_option() {
            crate::transaction_signature_options::SignWith::SignWithPlaintextPrivateKey(
                sign_private_key,
            ) => {
                sign_private_key
                    .process(
                        prepopulated_unsigned_transaction,
                        self.network.get_connection_config(),
                    )
                    .await
            }
            crate::transaction_signature_options::SignWith::SignWithKeychain(sign_keychain) => {
                sign_keychain
                    .process(
                        prepopulated_unsigned_transaction,
                        self.network.get_connection_config(),
                    )
                    .await
            }
            crate::transaction_signature_options::SignWith::SignWithLedger(sign_ledger) => {
                sign_ledger
                    .process(
                        prepopulated_unsigned_transaction,
                        self.network.get_connection_config(),
                    )
                    .await
            }
        }
    }
}
