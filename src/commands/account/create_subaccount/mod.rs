mod add_full_access_key;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct SubAccount {
    ///What is the receiver account ID?
    new_account_id: crate::types::account_id::AccountId,
    ///Enter the amount for the subaccount
    initial_balance: crate::common::NearBalance,
    #[interactive_clap(named_arg)]
    ///Specify a full access key for the sub-account
    pub sub_account_full_access: self::add_full_access_key::SubAccountFullAccess,
}

impl SubAccount {
    pub async fn process(&self) -> crate::CliResult {
        let prepopulated_unsigned_transaction = near_primitives::transaction::Transaction {
            signer_id: self
                .new_account_id
                .clone()
                .get_owner_account_id_from_sub_account()
                .into(),
            public_key: near_crypto::PublicKey::empty(near_crypto::KeyType::ED25519),
            nonce: 0,
            receiver_id: self.new_account_id.clone().into(),
            block_hash: Default::default(),
            actions: vec![
                near_primitives::transaction::Action::CreateAccount(
                    near_primitives::transaction::CreateAccountAction {},
                ),
                near_primitives::transaction::Action::Transfer(
                    near_primitives::transaction::TransferAction {
                        deposit: self.initial_balance.to_yoctonear(),
                    },
                ),
            ],
        };
        self.sub_account_full_access
            .process(prepopulated_unsigned_transaction)
            .await
    }
}
