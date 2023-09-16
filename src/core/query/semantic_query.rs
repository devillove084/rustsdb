use std::collections::HashMap;

use crate::common::{
    data::timestamp::TimeStamp,
    query::{
        filter::{named_filter::NamedFilter, query_filter::QueryFilter},
        query_mode::QueryMode,
        query_node_config::QueryNodeConfig,
        serdes::serdes_options::SerdesOptions,
        time_series_query::{CacheMode, LogLevel, TimeSeriesQuery},
    },
};

pub(crate) struct SemanticQuery {
    start: String,
    start_ts: Box<dyn TimeStamp>,
    end: String,
    end_ts: Box<dyn TimeStamp>,
    time_zone: String,
    execution_graph: Vec<Box<dyn QueryNodeConfig>>,
    filters: HashMap<String, Box<dyn NamedFilter>>,
    query_mode: QueryMode,
    cache_mode: CacheMode,
    log_level: LogLevel,
    cached_hash: u64,
}

impl TimeSeriesQuery for SemanticQuery {
    fn get_start(&self) -> String {
        todo!()
    }

    fn get_end(&self) -> String {
        todo!()
    }

    fn get_timezone(&self) -> String {
        todo!()
    }

    fn get_mode(&self) -> crate::common::query::query_mode::QueryMode {
        todo!()
    }

    fn get_cache_mode(&self) -> CacheMode {
        todo!()
    }

    fn get_filters(&self) -> Vec<Box<dyn NamedFilter>> {
        todo!()
    }

    fn get_filter(&self, filter_id: String) -> Box<dyn QueryFilter> {
        todo!()
    }

    fn start_time(&self) -> Box<dyn TimeStamp> {
        todo!()
    }

    fn end_time(&self) -> Box<dyn TimeStamp> {
        todo!()
    }

    fn get_serdes_configs(&self) -> Vec<Box<dyn SerdesOptions>> {
        todo!()
    }

    fn get_log_level(&self) -> LogLevel {
        todo!()
    }

    fn is_trace_enable(&self) -> bool {
        todo!()
    }

    fn is_debug_enable(&self) -> bool {
        todo!()
    }

    fn is_warn_enable(&self) -> bool {
        todo!()
    }

    fn get_execution_graph(&self) -> Vec<Box<dyn QueryNodeConfig>> {
        todo!()
    }
}
