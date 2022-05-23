mod tokens;

#[derive(clap::Subcommand, Debug, Clone)]
pub enum TopLevel {
    Account(Account),
    Contract(Contract),
    Tokens(tokens::TokensCommands),
}

impl TopLevel {
    pub async fn process(&self) -> crate::CliResult {
        let unsigned_transaction = near_primitives::transaction::Transaction {
            signer_id: "test".parse().unwrap(),
            public_key: near_crypto::PublicKey::empty(near_crypto::KeyType::ED25519),
            nonce: 0,
            receiver_id: "test".parse().unwrap(),
            block_hash: Default::default(),
            actions: vec![],
        };
        match self {
            Self::Tokens(tokens_commands) => tokens_commands.process(unsigned_transaction).await,
            _ => todo!(),
        }
    }
}

#[derive(clap::Args, Debug, Clone)]
struct Account {
    account_id: String,
    #[clap(subcommand)]
    network: NetworkArg<ViewAtBlock>,
}

#[derive(clap::Args, Debug, Clone)]
struct Contract {
    contract_id: String,
    #[clap(subcommand)]
    network: NetworkArg<crate::transaction_signature_options::SignWith>,
}

#[derive(clap::Subcommand, Debug, Clone)]
enum NetworkArg<Next: clap::Subcommand> {
    Network(Network<Next>),
}

impl<Next: clap::Subcommand + std::fmt::Debug> NetworkArg<Next> {
    pub async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        match self {
            Self::Network(network) => network.process(prepopulated_unsigned_transaction).await,
        }
    }
}

#[derive(clap::Args, Debug, Clone)]
struct Network<Next: clap::Subcommand> {
    network_name: String,
    #[clap(subcommand)]
    next: Next,
}

impl<Next: clap::Subcommand + std::fmt::Debug> Network<Next> {
    async fn process(
        &self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        let connection_config: crate::common::ConnectionConfig = match self.network_name.as_str() {
            "testnet" => crate::common::ConnectionConfig::Testnet,
            "mainnet" => crate::common::ConnectionConfig::Mainnet,
            "betanet" => crate::common::ConnectionConfig::Betanet,
            _ => todo!(),
        };

        println!("*****  self: {:#?}", self);
        match &self.next {
            // crate::transaction_signature_options::SignWith::SignWithPlaintextPrivateKey(
            //     sign_private_key,
            // ) => sign_private_key.process().await,
            _ => crate::transaction_signature_options::SignWith::process(
                &crate::transaction_signature_options::SignWith::SignWithPlaintextPrivateKey(
                    crate::transaction_signature_options::sign_with_private_key::SignPrivateKey {
                        signer_public_key: "ed25519:EAvya9ABXCaPv8rU1rnxd9xThXN6guAFVVBuvaXNWg8G"  // owner_account_id: volodymyr.testnet
                            .parse()?,
                        signer_private_key: "ed25519:2n9y3EZZUf4y9HZmkRZTmRHJ1ihKfPSbeBgUUCREqkcmwvJh1xXwgHaw4r1fs4hCLNNwC6ZN43hv83rDVyCP1h84"
                            .parse()?,
                        nonce: None,
                        block_hash: None,
                        submit: crate::transaction_signature_options::Submit::Send,
                    },
                ),
                prepopulated_unsigned_transaction,
                connection_config
            )
            .await,
        }
    }
}

#[derive(clap::Subcommand, Debug, Clone)]
enum ViewAtBlock {
    Now,
    AtBlockHeight(AtBlockHeight),
}

impl ViewAtBlock {
    async fn process(&self) -> crate::CliResult {
        match self {
            _ => todo!(),
        }
    }
}

#[derive(clap::Args, Debug, Clone)]
struct AtBlockHeight {
    block_height: u64,
}
