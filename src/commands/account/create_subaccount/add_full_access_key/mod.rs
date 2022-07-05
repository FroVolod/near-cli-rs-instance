use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod autogenerate_new_keypair;
mod use_manually_provided_seed_prase;
mod use_public_key;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct SubAccountFullAccess {
    #[interactive_clap(subcommand)]
    pub access_key_mode: AccessKeyMode,
}

impl SubAccountFullAccess {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
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
    UseManuallyProvidedSeedPrase(
        self::use_manually_provided_seed_prase::AddAccessWithSeedPhraseAction,
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
            AccessKeyMode::UseManuallyProvidedSeedPrase(add_access_with_seed_phrase_action) => {
                add_access_with_seed_phrase_action
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
        }
    }
}
