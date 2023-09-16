use super::{
    query_node_config::QueryNodeConfig, query_node_factory::QueryNodeFactory,
    query_pipeline_context::QueryPipelineContext, query_result::QueryResult,
};
use crate::common::{data::partial_time_series::PartialTimeSeries, stats::span::Span};

#[async_trait::async_trait]
pub(crate) trait QueryNode {
    async fn factory(&self) -> Box<dyn QueryNodeFactory>;

    async fn pipeline_context(&self) -> Box<dyn QueryPipelineContext>;

    async fn initialize(&self, span: Box<dyn Span + Send>);

    fn config(&self) -> Box<dyn QueryNodeConfig>;

    async fn close(&self);

    async fn on_complete(
        &self,
        downstream: Box<dyn QueryNode + Send>,
        final_seq: u64,
        total_seq: u64,
    );

    async fn on_next(&self, next: Box<dyn QueryResult + Send>);

    async fn on_next_by_partial(&self, next: Box<dyn PartialTimeSeries + Send>);

    async fn on_error(&self);
}
