use std::collections::HashMap;

use dyn_clone::{clone_trait_object, DynClone};

use super::query_sink_config::QuerySinkConfig;
use super::time_series_query::TimeSeriesQuery;
use super::{query_mode::QueryMode, query_sink::QuerySink};
use crate::common::stats::query_stats::QueryStats;
use crate::common::{core::tsdb::TSDB, data::time_series_byte_id::TimeSeriesID, stats::span::Span};

#[async_trait::async_trait]
pub(crate) trait QueryContext: DynClone {
    async fn sinks(&self) -> Vec<Box<dyn QuerySink>>;

    async fn mode(&self) -> QueryMode;

    async fn fetch_next(&self, span: Box<dyn Span>);

    async fn close(&self);

    fn is_closed(&self) -> bool;

    async fn stats(&self) -> Box<dyn QueryStats>;

    async fn sink_configs(&self) -> Vec<Box<dyn QuerySinkConfig>>;

    async fn query(&self) -> Box<dyn TimeSeriesQuery>;

    async fn tsdb(&self) -> Box<dyn TSDB>;

    // async fn auth_state(&self) -> AuthState;

    async fn headers(&self) -> HashMap<String, String>;

    fn cacheable(&self) -> bool;

    async fn initialize(&self, span: Box<dyn Span>);

    fn get_id(&self, hash: u64) -> Box<dyn TimeSeriesID>;

    // TODO: log opteartions!!
}

clone_trait_object!(QueryContext);
