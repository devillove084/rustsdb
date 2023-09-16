use std::hash::Hash;

use super::{
    filter::{named_filter::NamedFilter, query_filter::QueryFilter},
    query_mode::QueryMode,
    query_node_config::{Builder, QueryNodeConfig},
    serdes::serdes_options::SerdesOptions,
};
use crate::common::data::timestamp::TimeStamp;

pub(crate) enum LogLevel {
    OFF,
    ERROR,
    WARN,
    INFO,
    DEBUG,
    TRACE,
}

pub(crate) enum CacheMode {
    NORMAL,
    READONLY,
    WRITEONLY,
    BYPASS,
    CLEAR,
}

pub(crate) trait TimeSeriesQuery {
    fn get_start(&self) -> String;

    fn get_end(&self) -> String;

    fn get_timezone(&self) -> String;

    fn get_mode(&self) -> QueryMode;

    fn get_cache_mode(&self) -> CacheMode;

    fn get_filters(&self) -> Vec<Box<dyn NamedFilter>>;

    fn get_filter(&self, filter_id: String) -> Box<dyn QueryFilter>;

    fn start_time(&self) -> Box<dyn TimeStamp>;

    fn end_time(&self) -> Box<dyn TimeStamp>;

    fn get_execution_graph(&self) -> Vec<Box<dyn QueryNodeConfig>>;

    fn get_serdes_configs(&self) -> Vec<Box<dyn SerdesOptions>>;

    fn get_log_level(&self) -> LogLevel;

    fn is_trace_enable(&self) -> bool;

    fn is_debug_enable(&self) -> bool;

    fn is_warn_enable(&self) -> bool;
}

impl PartialEq for dyn TimeSeriesQuery {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl Eq for dyn TimeSeriesQuery {}

impl PartialOrd for dyn TimeSeriesQuery {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl Hash for dyn TimeSeriesQuery {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        todo!()
    }
}
