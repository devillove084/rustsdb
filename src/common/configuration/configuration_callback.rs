use bytes::Bytes;

use super::error::ConfigurationResult;

#[async_trait::async_trait]
pub(crate) trait ConfigurationCallback {
    async fn update(&self, key: String, value: Bytes) -> ConfigurationResult<()>;
}
