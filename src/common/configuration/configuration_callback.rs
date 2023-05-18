use super::error::ConfigurationResult;

#[async_trait::async_trait]
pub(crate) trait ConfigurationCallback<T> {
    async fn update(&self, key: String, value: T) -> ConfigurationResult<T>;
}
