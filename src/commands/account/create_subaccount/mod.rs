use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod autogenerate_new_keypair;
mod use_manually_provided_seed_phrase;
mod use_public_key;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct SubAccount {
    ///What is the receiver account ID?
    new_account_id: crate::types::account_id::AccountId,
    ///Enter the amount for the subaccount
    initial_balance: crate::common::NearBalance,
    #[interactive_clap(subcommand)]
    pub access_key_mode: AccessKeyMode,
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
        self.access_key_mode
            .process(prepopulated_unsigned_transaction)
            .await
    }
}

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Add a full access key for the sub-account
pub enum AccessKeyMode {
    #[strum_discriminants(strum(message = "Automatically generate a key pair"))]
    ///Automatically generate a key pair
    AutogenerateNewKeypair(self::autogenerate_new_keypair::GenerateKeypair),
    #[strum_discriminants(strum(message = "Use the provided seed phrase manually"))]
    ///Use the provided seed phrase manually
    UseManuallyProvidedSeedPhrase(
        self::use_manually_provided_seed_phrase::AddAccessWithSeedPhraseAction,
    ),
    #[strum_discriminants(strum(message = "Use the provided public key manually"))]
    ///Use the provided public key manually
    UseManuallyProvidedPublicKey(self::use_public_key::AddAccessKeyAction),
}

impl AccessKeyMode {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        match self {
            AccessKeyMode::UseManuallyProvidedPublicKey(add_access_key_action) => {
                add_access_key_action
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
            AccessKeyMode::AutogenerateNewKeypair(generate_keypair) => {
                generate_keypair
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
            AccessKeyMode::UseManuallyProvidedSeedPhrase(add_access_with_seed_phrase_action) => {
                add_access_with_seed_phrase_action
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
        }
    }
}
