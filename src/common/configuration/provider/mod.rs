pub mod base_provider;
pub mod map_provider;
pub mod provider_factory;
pub mod secret_provider;

use std::collections::HashMap;

use self::provider_factory::ProviderFactory;

use super::configuration_override::ConfigurationOverride;

#[async_trait::async_trait]
pub(crate) trait Provider {
    async fn get_setting(&self, key: String) -> ConfigurationOverride;
    async fn source(&self) -> String;
    async fn reload(&self);
    async fn last_reload(&self) -> u64;
    async fn factory(&self) -> Box<dyn ProviderFactory>;
    async fn populate_raw_map(&self, map: HashMap<String, String>);
}
