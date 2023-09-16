use std::{
    collections::{HashMap, HashSet},
    todo,
};

use hashed_wheel_timer::WheelTimer;

use super::{
    configuration_entry::ConfigurationEntry,
    provider::{
        map_provider::MapProvider, provider_factory::ProviderFactory,
        secret_provider::SecretProvider, Provider,
    },
};

pub const CONFIG_PROVIDER_KEY: &str = "config.providers";
pub const CONFIG_PROVIDERS_DEFAULT: &str =
    "PropertiesFile,Environment,SystemProperties,CommandLine,RuntimeOverride";
pub const CONFIG_PROVIDERS_REX: &str = r"^[\w+\d+,\/\.:\|\\\- ]+$";

pub const PLUGIN_DIRECTORY_KEY: &str = "config.plugin.path";
pub const PLUGIN_DIRECTORY_REX: &str = r"^[\w+\d+\/\.\\:\-]+$";

pub const CONFIG_RELOAD_INTERVAL_KEY: &str = "config.reload.interval";

pub struct Configuration {
    merged_config: HashMap<String, ConfigurationEntry>,
    providers: Vec<Box<dyn Provider>>,
    secret_providers: HashMap<String, Box<dyn SecretProvider>>,
    provider_config: String,
    provider_path: String,
    factories: Vec<Box<dyn ProviderFactory>>,
    reload_keys: HashSet<String>,
    timer: WheelTimer,
}

impl Clone for Configuration {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Configuration {
    pub fn new() -> Self {
        let timer = WheelTimer::new(0, 0).unwrap();

        Configuration {
            merged_config: HashMap::new(),
            providers: Vec::new(),
            secret_providers: HashMap::new(),
            provider_config: "".to_string(),
            provider_path: "".to_string(),
            factories: Vec::new(),
            reload_keys: HashSet::new(),
            timer,
        }
    }

    pub fn get_port(&self) -> i32 {
        todo!()
    }

    pub fn get_ssl_port(&self) -> i32 {
        todo!()
    }

    pub fn get_bind(&self) -> String {
        todo!()
    }

    pub fn get_root(&self) -> String {
        todo!()
    }

    pub fn get_load_plugins(&self) -> bool {
        todo!()
    }

    pub fn has_property(&self, key: String) -> bool {
        if key.is_empty() {
            // TODO: Exception, key cannot be null or empty
            return false;
        }
        match self.merged_config.get(&key) {
            Some(e) => return e.schema().is_null(),
            None => return false,
        }
    }

    pub fn register_by_string_value(
        &self,
        key: String,
        default_value: String,
        is_dynamic: bool,
        description: String,
    ) {
        todo!()
    }

    pub fn register_by_bool_value(
        &self,
        key: String,
        default_value: bool,
        is_dynamic: bool,
        description: String,
    ) {
        todo!()
    }

    pub fn register_by_int_value(
        &self,
        key: String,
        default_value: i32,
        is_dynamic: bool,
        description: String,
    ) {
        todo!()
    }

    pub fn get_int(&self, key: String) -> i32 {
        todo!()
    }

    // pub async fn new_configuration_with_map_provider(
    //     &self,
    //     cli_args: Vec<String>,
    //     map_provider: MapProvider,
    // ) -> Self { todo!()
    // }
}
