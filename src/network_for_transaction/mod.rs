use dialoguer::{theme::ColorfulTheme, Input, Select};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = crate::GlobalContext)]
pub struct NetworkForTransactionArgs {
    ///What is the name of the network
    #[interactive_clap(skip_default_input_arg)]
    network_name: String,
    #[interactive_clap(subcommand)]
    transaction_signature_options: crate::transaction_signature_options::SignWith,
}

impl NetworkForTransactionArgs {
    fn input_network_name(context: &crate::GlobalContext) -> color_eyre::eyre::Result<String> {
        println!("---  context: {:#?}", &context.0.networks);
        let variants = context.0.networks.keys().collect::<Vec<_>>();
        // let submits = variants
        //     .iter()
        //     .map(|p| p.get_message().unwrap().to_owned())
        //     .collect::<Vec<_>>();
        let select_submit = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What is the name of the network?")
            .items(&variants)
            .default(0)
            .interact()
            .unwrap();
        let url = context.0.networks.get(variants[select_submit]).unwrap().url.clone();
        Ok(variants[select_submit].to_string())
        // Ok(Input::new()
        //     .with_prompt("What is the name of the network?")
        //     .interact_text()?)
    }

    pub fn get_connection_config(
        &self,
        config: crate::config::Config,
    ) -> crate::common::ConnectionConfig {
        match self.network_name.as_str() {
            "testnet" => crate::common::ConnectionConfig::Testnet,
            "mainnet" => crate::common::ConnectionConfig::Mainnet,
            "betanet" => crate::common::ConnectionConfig::Betanet,
            _ => todo!(),
        }
    }

    pub fn get_sign_option(&self) -> crate::transaction_signature_options::SignWith {
        self.transaction_signature_options.clone()
    }
}
