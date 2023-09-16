use std::collections::HashMap;

use anyhow::Result;
use hashed_wheel_timer::WheelTimer;

use crate::common::{
    configuration::configuration::Configuration,
    core::{registry::Registry, tsdb::TSDB},
    pool::executor::ExecutorService,
    query::query_context::QueryContext,
    stats::stats_collector::StatsCollector,
    threadpools::tsdb_thread_pool_executor::TSDBThreadPoolExecutor,
};

pub const MAINT_TIMER_KEY: &str = "tsd.maintenance.frequency";
pub const MAINT_TIMER_DEFAULT: i32 = 60000;

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct DefaultTSDB {
    executor: ExecutorService,

    config: Configuration,

    registry: Box<dyn Registry>,

    stats_collector: Box<dyn StatsCollector>,

    maintenance_timer: WheelTimer,

    query_timer: WheelTimer,

    query_pool: Box<dyn TSDBThreadPoolExecutor>,

    running_queries: HashMap<u64, Box<dyn QueryContext>>,
}

unsafe impl Sync for DefaultTSDB {}

unsafe impl Send for DefaultTSDB {}

#[async_trait::async_trait]
impl TSDB for DefaultTSDB {
    fn get_config(&self) -> Configuration {
        self.config.clone()
    }

    fn get_registry(&self) -> Box<dyn Registry> {
        self.registry.clone()
    }

    fn get_stats_collector(&self) -> Box<dyn StatsCollector> {
        self.stats_collector.clone()
    }

    fn get_maintenance_timer(&self) -> WheelTimer {
        self.maintenance_timer.clone()
    }

    fn get_query_thread_pool(&self) -> Box<dyn TSDBThreadPoolExecutor> {
        self.query_pool.clone()
    }

    fn quick_work_pool(&self) -> ExecutorService {
        self.executor.clone()
    }

    fn get_query_timer(&self) -> WheelTimer {
        self.query_timer.clone()
    }

    async fn shutdown(&self) {
        // TODO: Should be more eligent
        todo!()
        // let _ = self.registry.shutdown();
    }

    fn register_running_query(&self, _hash: u64, _context: Box<dyn QueryContext>) -> bool {
        todo!()
    }

    fn complete_running_query(&self, _hash: u64) -> bool {
        todo!()
    }
}

#[allow(dead_code)]
impl DefaultTSDB {
    pub fn new(_config: Configuration) -> Self {
        todo!()
    }

    pub async fn initialize_registry(&self, _load_plugin: bool) -> Result<()> {
        todo!()
    }
}
