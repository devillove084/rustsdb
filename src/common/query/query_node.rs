use crate::common::{data::partial_time_series::PartialTimeSeries, stats::span::Span};

use super::{
    query_node_config::{Builder, QueryNodeConfig},
    query_node_factory::QueryNodeFactory,
    query_pipeline_context::QueryPipelineContext,
    query_result::QueryResult,
};

#[async_trait::async_trait]
pub(crate) trait QueryNode<B, C>
where
    C: QueryNodeConfig<B, C>,
    B: Builder<B, C>,
{
    async fn factory(&self) -> Box<dyn QueryNodeFactory<B, C>>;

    async fn pipeline_context(&self) -> Box<dyn QueryPipelineContext<B, C>>;

    async fn initialize(&self, span: Box<dyn Span>);

    fn config(&self) -> Box<dyn QueryNodeConfig<B, C>>;

    async fn close(&self);

    async fn on_complete(
        &self,
        downstream: Box<dyn QueryNode<B, C>>,
        final_seq: u64,
        total_seq: u64,
    );

    async fn on_next(&self, next: Box<dyn QueryResult>);

    async fn on_next_by_partial(&self, next: Box<dyn PartialTimeSeries>);

    async fn on_error(&self);
}
