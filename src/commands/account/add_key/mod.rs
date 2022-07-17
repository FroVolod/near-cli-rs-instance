use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod access_key_type;
mod autogenerate_new_keypair;
mod use_manually_provided_seed_phrase;
mod use_public_key;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct AddKeyCommand {
    ///Which account should You add an access key to?
    owner_account_id: crate::types::account_id::AccountId,
    #[interactive_clap(subcommand)]
    permission: AccessKeyPermission,
}

impl AddKeyCommand {
    pub async fn process(&self) -> crate::CliResult {
        let prepopulated_unsigned_transaction = near_primitives::transaction::Transaction {
            signer_id: self.owner_account_id.clone().into(),
            public_key: near_crypto::PublicKey::empty(near_crypto::KeyType::ED25519),
            nonce: 0,
            receiver_id: self.owner_account_id.clone().into(),
            block_hash: Default::default(),
            actions: vec![],
        };
        match self.permission.clone() {
            AccessKeyPermission::GrantFullAccess(full_access_type) => {
                full_access_type
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
            AccessKeyPermission::GrantFunctionCallAccess(function_call_type) => {
                function_call_type
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
        }
    }
}

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Select a permission that you want to add to the access key
pub enum AccessKeyPermission {
    #[strum_discriminants(strum(message = "A permission with function call"))]
    /// Provide data for a function-call access key
    GrantFunctionCallAccess(self::access_key_type::FunctionCallType),
    #[strum_discriminants(strum(message = "A permission with full access"))]
    /// Provide data for a full access key
    GrantFullAccess(self::access_key_type::FullAccessType),
}

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Add an access key for this account
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
        permission: near_primitives::account::AccessKeyPermission,
    ) -> crate::CliResult {
        match self {
            AccessKeyMode::UseManuallyProvidedPublicKey(add_access_key_action) => {
                add_access_key_action
                    .process(prepopulated_unsigned_transaction, permission)
                    .await
            }
            AccessKeyMode::AutogenerateNewKeypair(generate_keypair) => {
                generate_keypair
                    .process(prepopulated_unsigned_transaction, permission)
                    .await
            }
            AccessKeyMode::UseManuallyProvidedSeedPhrase(add_access_with_seed_phrase_action) => {
                add_access_with_seed_phrase_action
                    .process(prepopulated_unsigned_transaction, permission)
                    .await
            }
        }
    }
}
