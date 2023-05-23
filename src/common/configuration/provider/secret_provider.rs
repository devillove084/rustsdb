use bytes::Bytes;
use hashed_wheel_timer::WheelTimer;

use crate::common::configuration::Configuration;

use super::provider_factory::ProviderFactory;

#[async_trait::async_trait]
pub(crate) trait SecretProvider {
    async fn get_secret_string(&self, key: String) -> String;
    async fn get_secret_byte(&self, key: String) -> Bytes;
    async fn get_secret_obj(&self, key: String) -> Box<dyn SecretProvider>;
    async fn initialize(
        &self,
        factory: Box<dyn ProviderFactory>,
        config: Configuration,
        timer: WheelTimer,
        id: String,
    );
}
