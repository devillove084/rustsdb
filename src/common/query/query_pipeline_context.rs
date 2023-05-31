use crate::common::{
    core::tsdb::TSDB,
    data::{
        time_series_byte_id::TimeSeriesID, time_series_data_source::TimeSeriesDataSource,
        time_series_data_source_config::TimeSeriesDataSourceConfig,
    },
    stats::span::Span,
};

use super::{
    query_context::QueryContext,
    query_node::QueryNode,
    query_node_config::{Builder, QueryNodeConfig},
    query_node_factory::QueryNodeFactory,
    query_result_id::QueryResultID,
    query_sink::QuerySink,
    time_series_query::TimeSeriesQuery,
};

#[async_trait::async_trait]
pub(crate) trait QueryPipelineContext<B, C>
where
    Self: QueryNode<B, C>,
    B: Builder<B, C>,
    C: QueryNodeConfig<B, C>,
{
    async fn factory(&self) -> Box<dyn QueryNodeFactory<B, C>>;

    fn tsdb(&self) -> Box<dyn TSDB>;

    async fn query(&self) -> Box<dyn TimeSeriesQuery>;

    async fn query_context(&self) -> Box<dyn QueryContext>;

    async fn initialize(&self, span: Box<dyn Span>);

    async fn fetch_next(&self, span: Box<dyn Span>);

    async fn upstream(&self, node: Box<dyn QueryNode<B, C>>) -> Vec<Box<dyn QueryNode<B, C>>>;

    async fn downstream(&self, node: Box<dyn QueryNode<B, C>>) -> Vec<Box<dyn QueryNode<B, C>>>;

    async fn downstream_sources(
        &self,
        node: Box<dyn QueryNode<B, C>>,
    ) -> Vec<Box<dyn TimeSeriesDataSource<B, C>>>;

    async fn common_source_config(
        &self,
        node: Box<dyn QueryNode<B, C>>,
    ) -> Box<dyn TimeSeriesDataSourceConfig<B, C>>;

    async fn downstream_sources_ids(&self, node: Box<dyn QueryNode<B, C>>) -> Vec<String>;

    async fn downstream_sources_result_ids(
        &self,
        node: Box<dyn QueryNode<B, C>>,
    ) -> Vec<Box<dyn QueryResultID>>;

    async fn upstream_of_type(
        &self,
        node: Box<dyn QueryNode<B, C>>,
    ) -> Vec<Box<dyn QueryNode<B, C>>>;

    async fn downstream_of_type(
        &self,
        node: Box<dyn QueryNode<B, C>>,
    ) -> Vec<Box<dyn QueryNode<B, C>>>;

    async fn sinks(&self) -> Vec<Box<dyn QuerySink>>;

    async fn add_id(&self, hash: u64, id: Box<dyn TimeSeriesID>);

    async fn get_id(&self, hash: u64) -> Box<dyn TimeSeriesID>;

    async fn has_id(&self, hash: u64) -> bool;

    async fn close(&self);
}
