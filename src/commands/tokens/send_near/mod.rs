#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct SendNearCommand {
    ///What is the receiver account ID?
    receiver_account_id: crate::types::account_id::AccountId,
    ///Enter an amount to transfer
    amount_in_near: crate::common::NearBalance,
    #[interactive_clap(named_arg)]
    ///Select online mode
    network: crate::network_for_transaction::NetworkForTransactionArgs,
}

impl SendNearCommand {
    pub async fn process(
        &self,
        owner_account_id: near_primitives::types::AccountId,
    ) -> crate::CliResult {
        let prepopulated_unsigned_transaction = near_primitives::transaction::Transaction {
            signer_id: owner_account_id,
            public_key: near_crypto::PublicKey::empty(near_crypto::KeyType::ED25519),
            nonce: 0,
            receiver_id: self.receiver_account_id.clone().into(),
            block_hash: Default::default(),
            actions: vec![near_primitives::transaction::Action::Transfer(
                near_primitives::transaction::TransferAction {
                    deposit: self.amount_in_near.to_yoctonear(),
                },
            )],
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
