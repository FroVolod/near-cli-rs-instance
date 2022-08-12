use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = crate::GlobalContext)]
#[interactive_clap(skip_default_from_cli)]
pub struct AddNetworkConnection {
    ///What is the network name?
    network_name: String,
    ///What is the RPC endpoint?
    rpc_url: crate::common::AvailableRpcServerUrl,
    ///What is the wallet endpoint?
    wallet_url: crate::common::AvailableRpcServerUrl,
    ///What is the transaction explorer endpoint?
    explorer_transaction_url: crate::common::AvailableRpcServerUrl,
    #[interactive_clap(skip_default_from_cli_arg)]
    #[interactive_clap(skip_default_input_arg)]
    api_key: Option<String>,
}

impl AddNetworkConnection {
    pub fn from_cli(
        optional_clap_variant: Option<
            <AddNetworkConnection as interactive_clap::ToCli>::CliVariant,
        >,
        context: crate::GlobalContext,
    ) -> color_eyre::eyre::Result<Self> {
        let network_name = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.network_name)
        {
            Some(cli_network_name) => cli_network_name,
            None => Self::input_network_name(&context)?,
        };
        let rpc_url = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.rpc_url)
        {
            Some(cli_rpc_url) => cli_rpc_url,
            None => Self::input_rpc_url(&context)?,
        };
        let wallet_url = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.wallet_url)
        {
            Some(cli_wallet_url) => cli_wallet_url,
            None => Self::input_wallet_url(&context)?,
        };
        let explorer_transaction_url = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.explorer_transaction_url)
        {
            Some(cli_explorer_transaction_url) => cli_explorer_transaction_url,
            None => Self::input_explorer_transaction_url(&context)?,
        };
        let api_key: Option<String> = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.api_key)
        {
            Some(cli_api_key) => Some(cli_api_key),
            None => Self::input_api_key()?,
        };
        Ok(Self {
            network_name,
            rpc_url,
            wallet_url,
            explorer_transaction_url,
            api_key,
        })
    }

    fn input_api_key() -> color_eyre::eyre::Result<Option<String>> {
        println!();
        let choose_input = vec![
            "Yes, I want to input api_key",
            "No, I don't to input api_key",
        ];
        let select_choose_input = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Do You want to input an api key?")
            .items(&choose_input)
            .default(0)
            .interact_on_opt(&Term::stderr())?;
        match select_choose_input {
            Some(0) => {
                let api_key = Input::new()
                    .with_prompt("Enter an api key)")
                    .interact_text()?;
                Ok(Some(api_key))
            }
            Some(1) => Ok(None),
            _ => unreachable!("Error"),
        }
    }

    pub async fn process(&self, mut config: crate::config::Config) -> crate::CliResult {
        config.networks.insert(
            self.network_name.clone(),
            crate::config::NetworkConfig {
                network_name: self.network_name.clone(),
                rpc_url: self.rpc_url.inner.clone(),
                wallet_url: self.wallet_url.inner.clone(),
                explorer_transaction_url: self.explorer_transaction_url.inner.clone(),
                api_key: self.api_key.clone(),
            },
        );
        crate::common::write_config_toml(config)
    }
}
