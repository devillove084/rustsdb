use crate::common::core::tsdb_plugin::TSDBPlugin;

pub(crate) trait StatsTimer {
    fn stop(&self, tags: Vec<String>);
    fn start_time(&self) -> u64;
    fn units(&self) -> u64;
}

#[async_trait::async_trait]
pub(crate) trait StatsCollector: TSDBPlugin {
    async fn increment_counter(&self, metric: String, tags: Vec<String>);

    async fn increment_counter_with_amount(&self, metric: String, amount: u64, tags: Vec<String>);

    async fn set_gauge(&self, metric: String, value: u64, tags: Vec<String>);

    async fn start_timer(&self, metric: String, units: u64) -> Box<dyn StatsTimer>;

    async fn add_time(&self, metric: String, duration: u64, units: u64, tags: Vec<String>);
}
