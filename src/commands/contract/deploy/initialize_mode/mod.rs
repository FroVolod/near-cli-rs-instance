use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod call_function_type;

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap_derive::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum InitializeMode {
    /// Add an initialize
    #[strum_discriminants(strum(message = "Add an initialize"))]
    Initialize(self::call_function_type::CallFunctionAction),
    /// Don't add an initialize
    #[strum_discriminants(strum(message = "Don't add an initialize"))]
    NoInitialize(NoInitialize),
}

impl InitializeMode {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        match self {
            InitializeMode::Initialize(call_function_action) => {
                call_function_action
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
            InitializeMode::NoInitialize(no_initialize) => {
                no_initialize
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
        }
    }
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct NoInitialize {
    #[interactive_clap(named_arg)]
    ///Select online mode
    network: crate::network_for_transaction::NetworkForTransactionArgs,
}

impl NoInitialize {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
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
