use crate::common::{
    data::{partial_time_series::PartialTimeSeries, time_series_data_source::TimeSeriesDataSource},
    query::{
        query_node::QueryNode,
        query_node_config::{Builder, QueryNodeConfig},
        query_node_factory::QueryNodeFactory,
        query_pipeline_context::QueryPipelineContext,
        query_result::QueryResult,
    },
    stats::span::Span,
};

pub(crate) struct AbstractQueryNode {
    factory: Box<dyn QueryNodeFactory>,
    context: Box<dyn QueryPipelineContext>,
    upstream: Vec<Box<dyn QueryNode>>,
    downstream: Vec<Box<dyn QueryNode>>,
    downstream_sources: Vec<Box<dyn TimeSeriesDataSource>>,
}

unsafe impl Send for AbstractQueryNode {}

unsafe impl Sync for AbstractQueryNode {}

#[allow(unused_variables)]
#[allow(dead_code)]
impl AbstractQueryNode {
    pub fn new(factory: Box<dyn QueryNodeFactory>, context: Box<dyn QueryPipelineContext>) -> Self {
        todo!()
    }

    pub fn fetch_down_stream(&self, span: Box<dyn Span>) {
        todo!()
    }

    pub fn send_up_stream_with_ressult(&self, result: Box<dyn QueryResult>) {
        todo!()
    }

    pub fn send_up_stream_with_partial(&self, series: Box<dyn PartialTimeSeries>) {
        todo!()
    }

    // TODO: This is used to caught log or warning
    pub fn send_up_stream(&self) {
        todo!()
    }

    pub fn complete_up_stream(&self, final_seq: u64, total_seq: u64) {
        todo!()
    }
}

#[allow(unused_variables)]
#[async_trait::async_trait]
impl QueryNode for AbstractQueryNode {
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
