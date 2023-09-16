use std::{
    collections::HashMap,
    sync::atomic::{AtomicBool, AtomicI32},
};

use super::plan::default_query_planner::DefaultQueryPlanner;
use crate::common::{
    data::{partial_time_series::PartialTimeSeries, time_series_id::TimeSeriesID},
    query::{
        query_context::QueryContext,
        query_node::QueryNode,
        query_node_config::{Builder, QueryNodeConfig},
        query_node_factory::QueryNodeFactory,
        query_pipeline_context::QueryPipelineContext,
        query_result::QueryResult,
        query_result_id::QueryResultID,
        query_sink::QuerySink,
        query_sink_callback::QuerySinkCallback,
    },
    stats::span::Span,
};

struct PartialTimeSetWrapper {
    count: AtomicI32,
    max: AtomicI32,
}

pub(crate) struct AbstractQueryPipelineContext {
    sinks: Vec<Box<dyn QuerySink>>,
    count_downs: HashMap<Box<dyn QueryResultID>, AtomicI32>,
    context: Box<dyn QueryContext>,
    plan: DefaultQueryPlanner,
    complete: AtomicBool,
    source_idx: i32,
    pts: HashMap<String, HashMap<u64, PartialTimeSetWrapper>>,
    finished_sources: HashMap<String, AtomicI32>,
    total_finished: AtomicI32,
    ids: HashMap<Box<dyn TimeSeriesID>, HashMap<u64, Box<dyn TimeSeriesID>>>,
}

unsafe impl Send for AbstractQueryPipelineContext {}

unsafe impl Sync for AbstractQueryPipelineContext {}

#[async_trait::async_trait]
impl QueryNode for AbstractQueryPipelineContext {
    async fn initialize(&self, span: Box<dyn Span + Send>) {
        todo!()
    }

    async fn factory(&self) -> Box<dyn QueryNodeFactory> {
        todo!()
    }

    async fn pipeline_context(&self) -> Box<dyn QueryPipelineContext> {
        todo!()
    }

    fn config(&self) -> Box<dyn QueryNodeConfig> {
        todo!()
    }

    async fn close(&self) {
        todo!()
    }

    async fn on_complete(
        &self,
        down_stream: Box<dyn QueryNode + Send>,
        final_seq: u64,
        total_seq: u64,
    ) {
        todo!()
    }

    async fn on_next(&self, next: Box<dyn QueryResult + Send>) {
        todo!()
    }

    async fn on_next_by_partial(&self, next: Box<dyn PartialTimeSeries + Send>) {
        todo!()
    }

    async fn on_error(&self) {
        todo!()
    }
}

impl QueryPipelineContext for AbstractQueryPipelineContext {
    fn factory(&self) -> Box<dyn QueryNodeFactory> {
        todo!()
    }

    fn tsdb(&self) -> Box<dyn crate::common::core::tsdb::TSDB> {
        todo!()
    }

    fn query(&self) -> Box<dyn crate::common::query::time_series_query::TimeSeriesQuery> {
        todo!()
    }

    fn query_context(&self) -> Box<dyn QueryContext> {
        todo!()
    }

    fn initialize(&self, span: Box<dyn Span>) {
        todo!()
    }

    fn fetch_next(&self, span: Box<dyn Span>) {
        todo!()
    }

    fn upstream(&self, node: Box<dyn QueryNode>) -> Vec<Box<dyn QueryNode>> {
        todo!()
    }

    fn downstream(&self, node: Box<dyn QueryNode>) -> Vec<Box<dyn QueryNode>> {
        todo!()
    }

    fn downstream_sources(
        &self,
        node: Box<dyn QueryNode>,
    ) -> Vec<Box<dyn crate::common::data::time_series_data_source::TimeSeriesDataSource>> {
        todo!()
    }

    fn common_source_config(
        &self,
        node: Box<dyn QueryNode>,
    ) -> Box<dyn crate::common::data::time_series_data_source_config::TimeSeriesDataSourceConfig>
    {
        todo!()
    }

    fn downstream_sources_ids(&self, node: Box<dyn QueryNode>) -> Vec<String> {
        todo!()
    }

    fn downstream_sources_result_ids(
        &self,
        node: Box<dyn QueryNode>,
    ) -> Vec<Box<dyn QueryResultID>> {
        todo!()
    }

    fn upstream_of_type(&self, node: Box<dyn QueryNode>) -> Vec<Box<dyn QueryNode>> {
        todo!()
    }

    fn downstream_of_type(&self, node: Box<dyn QueryNode>) -> Vec<Box<dyn QueryNode>> {
        todo!()
    }

    fn sinks(&self) -> Vec<Box<dyn QuerySink>> {
        todo!()
    }

    fn add_id(&self, hash: u64, id: Box<dyn TimeSeriesID>) {
        todo!()
    }

    fn get_id(&self, hash: u64) -> Box<dyn TimeSeriesID> {
        todo!()
    }

    fn has_id(&self, hash: u64) -> bool {
        todo!()
    }

    fn close(&self) {
        todo!()
    }
}

impl QuerySinkCallback for AbstractQueryPipelineContext {
    fn on_complete(&self, pts: Box<dyn PartialTimeSeries>) {
        todo!()
    }

    fn on_error(&self, pts: Box<dyn PartialTimeSeries>) {
        todo!()
    }
}
