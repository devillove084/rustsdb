use std::{collections::HashMap, sync::atomic::AtomicBool};

use super::semantic_query::SemanticQuery;
use crate::common::{
    core::tsdb::TSDB,
    data::time_series_id::TimeSeriesID,
    query::{
        query_context::QueryContext,
        query_context_builder::QueryContextBuilder,
        query_mode::QueryMode,
        query_node_config::{Builder, QueryNodeConfig},
        query_pipeline_context::QueryPipelineContext,
        query_sink::QuerySink,
        query_sink_config::QuerySinkConfig,
        time_series_query::TimeSeriesQuery,
    },
    stats::{query_stats::QueryStats, span::Span},
};

pub(crate) struct BaseQueryContext {
    tsdb: Box<dyn TSDB>,
    query: SemanticQuery,
    stats: Box<dyn QueryStats>,
    sink_config: Vec<Box<dyn QuerySinkConfig>>,
    pipeline: Box<dyn QueryPipelineContext>,
    // auth_state: AuthState,
    headers: HashMap<String, String>,
    logs: Vec<String>,
    builder_sinks: Vec<Box<dyn QuerySink>>,
    local_span: Box<dyn Span>,
    is_closed: AtomicBool,
    cacheable: AtomicBool,
}

impl Clone for BaseQueryContext {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl QueryContext for BaseQueryContext {
    fn sinks(&self) -> Vec<Box<dyn QuerySink>> {
        todo!()
    }

    fn mode(&self) -> QueryMode {
        todo!()
    }

    fn fetch_next(&self, span: Box<dyn Span>) {
        todo!()
    }

    fn close(&self) {
        todo!()
    }

    fn is_closed(&self) -> bool {
        todo!()
    }

    fn stats(&self) -> Box<dyn QueryStats> {
        todo!()
    }

    fn sink_configs(&self) -> Vec<Box<dyn QuerySinkConfig>> {
        todo!()
    }

    fn query(&self) -> Box<dyn TimeSeriesQuery> {
        todo!()
    }

    fn tsdb(&self) -> Box<dyn TSDB> {
        todo!()
    }

    fn headers(&self) -> HashMap<String, String> {
        todo!()
    }

    fn cacheable(&self) -> bool {
        todo!()
    }

    fn initialize(&self, span: Box<dyn Span>) {
        todo!()
    }

    fn get_id(&self, hash: u64) -> Box<dyn TimeSeriesID> {
        todo!()
    }
}

pub(crate) struct LocalPipeline {}

impl LocalPipeline {
    pub fn new(context: Box<dyn QueryContext>, direct_sinks: Vec<Box<dyn QuerySink>>) -> Self {
        todo!()
    }
}

pub(crate) struct BaseBuilder {
    tsdb: Box<dyn TSDB>,
    query: SemanticQuery,
    stats: Box<dyn QueryStats>,
    sink_configs: Vec<Box<dyn QuerySinkConfig>>,
    sinks: Vec<Box<dyn QuerySink>>,
    // auth_state: AuthState,
    headers: HashMap<String, String>,
}

impl QueryContextBuilder for BaseBuilder {
    fn set_query(&mut self, query: Box<dyn TimeSeriesQuery>) {
        todo!()
    }

    fn set_mode(&mut self, mode: QueryMode) {
        todo!()
    }

    fn set_stats(&mut self, stats: Box<dyn QueryStats>) {
        todo!()
    }

    fn set_sinks(&mut self, configs: Vec<Box<dyn QuerySinkConfig>>) {
        todo!()
    }

    fn add_sink(&mut self, sink: Box<dyn QuerySink>) {
        todo!()
    }

    fn set_local_sinks(&mut self, sinks: Vec<Box<dyn QuerySink>>) {
        todo!()
    }

    fn set_header(&mut self, headers: HashMap<String, String>) {
        todo!()
    }

    fn build(&self) -> Box<dyn QueryContext> {
        todo!()
    }
}
