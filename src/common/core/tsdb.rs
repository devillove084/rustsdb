use dyn_clone::{clone_trait_object, DynClone};
use hashed_wheel_timer::WheelTimer;

use crate::common::configuration::Configuration;
use crate::common::core::registry::Registry;
use crate::common::pool::executor::ExecutorService;
use crate::common::query::query_context::QueryContext;
use crate::common::query::query_node_config::{Builder, QueryNodeConfig};
use crate::common::stats::stats_collector::StatsCollector;
use crate::common::threadpools::tsdb_thread_pool_executor::TSDBThreadPoolExecutor;
#[async_trait::async_trait]
#[allow(clippy::upper_case_acronyms)]
pub(crate) trait TSDB: DynClone + Send + Sync {
    fn get_config(&self) -> Configuration;

    fn get_registry(&self) -> Box<dyn Registry>;

    fn get_stats_collector(&self) -> Box<dyn StatsCollector>;

    fn get_maintenance_timer(&self) -> WheelTimer;

    fn get_query_thread_pool(&self) -> Box<dyn TSDBThreadPoolExecutor>;

    fn quick_work_pool(&self) -> ExecutorService;

    fn get_query_timer(&self) -> WheelTimer;

    async fn shutdown(&self);

    fn register_running_query(&self, hash: u64, context: Box<dyn QueryContext>) -> bool;

    fn complete_running_query(&self, hash: u64) -> bool;
}

clone_trait_object!(TSDB);
