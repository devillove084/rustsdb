use std::str::Bytes;

use crate::common::configuration::Configuration;

use super::provider_factory::ProviderFactory;

#[async_trait::async_trait]
pub(crate) trait SecretProvider<T> {
    async fn get_secret_string(&self, key: String) -> String;
    async fn get_secret_byte(&self, key: String) -> Bytes;
    async fn get_secret_obj(&self, key: String) -> T;
    async fn initialize(
        &self,
        factory: dyn ProviderFactory<T>,
        config: Configuration<T>,
        // timer: xxx
        id: String,
    );
}
