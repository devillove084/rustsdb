use std::collections::HashMap;

use dyn_clone::{clone_trait_object, DynClone};

use super::{
    query_mode::QueryMode, query_sink::QuerySink, query_sink_config::QuerySinkConfig,
    time_series_query::TimeSeriesQuery,
};
use crate::common::{
    core::tsdb::TSDB,
    data::time_series_id::TimeSeriesID,
    stats::{query_stats::QueryStats, span::Span},
};

pub(crate) trait QueryContext: DynClone {
    fn sinks(&self) -> Vec<Box<dyn QuerySink>>;

    fn mode(&self) -> QueryMode;

    fn fetch_next(&self, span: Box<dyn Span>);

    fn close(&self);

    fn is_closed(&self) -> bool;

    fn stats(&self) -> Box<dyn QueryStats>;

    fn sink_configs(&self) -> Vec<Box<dyn QuerySinkConfig>>;

    fn query(&self) -> Box<dyn TimeSeriesQuery>;

    fn tsdb(&self) -> Box<dyn TSDB>;

    // async fn auth_state(&self) -> AuthState;

    fn headers(&self) -> HashMap<String, String>;

    fn cacheable(&self) -> bool;

    fn initialize(&self, span: Box<dyn Span>);

    fn get_id(&self, hash: u64) -> Box<dyn TimeSeriesID>;

    // TODO: log opteartions!!
}

clone_trait_object!(QueryContext);
