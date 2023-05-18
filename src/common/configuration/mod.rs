use std::{
    collections::{HashMap, HashSet},
    todo,
};

use hashed_wheel_timer::WheelTimer;

use self::{
    configuration_entry::ConfigurationEntry,
    provider::{
        map_provider::MapProvider, provider_factory::ProviderFactory,
        secret_provider::SecretProvider, Provider,
    },
};

pub mod configuration_callback;
pub mod configuration_entry;
pub mod configuration_entry_schema;
pub mod configuration_override;
pub mod configuration_value_validator;
pub mod error;
pub mod provider;

pub const CONFIG_PROVIDER_KEY: &str = "config.providers";
pub const CONFIG_PROVIDERS_DEFAULT: &str =
    "PropertiesFile,Environment,SystemProperties,CommandLine,RuntimeOverride";
pub const CONFIG_PROVIDERS_REX: &str = r"^[\w+\d+,\/\.:\|\\\- ]+$";

pub const PLUGIN_DIRECTORY_KEY: &str = "config.plugin.path";
pub const PLUGIN_DIRECTORY_REX: &str = r"^[\w+\d+\/\.\\:\-]+$";

pub const CONFIG_RELOAD_INTERVAL_KEY: &str = "config.reload.interval";

pub struct Configuration<T> {
    merged_config: HashMap<String, ConfigurationEntry<T>>,
    providers: Vec<Box<dyn Provider<T>>>,
    secret_providers: HashMap<String, Box<dyn SecretProvider<T>>>,
    provider_config: String,
    provider_path: String,
    factories: Vec<Box<dyn ProviderFactory<T>>>,
    reload_keys: HashSet<String>,
    timer: WheelTimer,
}

impl<T> Configuration<T> {
    pub async fn new_configuration(&self, properties: HashMap<String, String>) -> Self {
        todo!()
    }

    pub async fn new_configuration_with_map_provider(
        &self,
        cli_args: Vec<String>,
        map_provider: MapProvider,
    ) -> Self {
        todo!()
    }
}
