use crate::common::configuration::Configuration;

use super::Provider;

#[async_trait::async_trait]
pub(crate) trait ProviderFactory<T> {
    // TODO: params need a more timer.
    // 'static may not be best choice
    async fn new_instance(&self, config: Configuration<T>) -> Box<dyn Provider<T> + 'static>;
    fn is_reloadable(&self) -> bool;
    fn description(&self) -> String;
    fn simple_name(&self) -> String;
}
