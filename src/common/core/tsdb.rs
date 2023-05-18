use std::future::Future;

use hashed_wheel_timer::WheelTimer;

use crate::common::configuration::Configuration;
use crate::common::core::registry::Registry;
use crate::common::query::query_context::QueryContext;
use crate::common::stats::stats_collector::StatsCollector;
use crate::common::threadpools::tsdb_thread_pool_executor::TSDBThreadPoolExecutor;
#[async_trait::async_trait]
#[allow(clippy::upper_case_acronyms)]
pub(crate) trait TSDB<T> {
    async fn get_config(&self) -> Configuration<T>;

    async fn get_registry(&self) -> impl Registry<T>;

    async fn get_stats_collector(&self) -> impl StatsCollector<T>;

    async fn get_maintenance_timer(&self) -> WheelTimer;

    async fn get_query_thread_pool(&self) -> impl TSDBThreadPoolExecutor<T>;

    // TODO: May not be a good choice.
    async fn quick_work_pool(&self) -> dyn Future<Output = T>;

    async fn get_query_timer(&self) -> WheelTimer;

    async fn shutdown(&self);

    // TODO: May query time servies id not u64.
    async fn register_running_query(
        &self,
        hash: u64,
        context: dyn QueryContext<T, QueryTimeSeriesID = u64>,
    ) -> bool;

    async fn complete_running_query(&self, hash: u64) -> bool;
}
