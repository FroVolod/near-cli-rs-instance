use dialoguer::{theme::ColorfulTheme, Select};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = crate::GlobalContext)]
pub struct DeleteNetworkConnection {
    ///What is the network name?
    #[interactive_clap(skip_default_input_arg)]
    network_name: String,
}

impl DeleteNetworkConnection {
    fn input_network_name(context: &crate::GlobalContext) -> color_eyre::eyre::Result<String> {
        let variants = context.0.networks.keys().collect::<Vec<_>>();
        let select_submit = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What is the name of the network?")
            .items(&variants)
            .default(0)
            .interact()
            .unwrap();
        Ok(variants[select_submit].to_string())
    }

    pub async fn process(&self, mut config: crate::config::Config) -> crate::CliResult {
        config.networks.remove(&self.network_name);
        crate::common::write_config_toml(config)
    }
}
