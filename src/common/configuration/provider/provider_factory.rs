use hashed_wheel_timer::WheelTimer;

use super::Provider;
use crate::common::configuration::configuration::Configuration;

#[async_trait::async_trait]
pub(crate) trait ProviderFactory {
    async fn new_instance(&self, config: Configuration, timer: WheelTimer) -> Box<dyn Provider>;
    fn is_reloadable(&self) -> bool;
    fn description(&self) -> String;
    fn simple_name(&self) -> String;
}
