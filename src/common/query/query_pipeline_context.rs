use super::{
    query_context::QueryContext,
    query_node::QueryNode,
    query_node_config::{Builder, QueryNodeConfig},
    query_node_factory::QueryNodeFactory,
    query_result_id::QueryResultID,
    query_sink::QuerySink,
    time_series_query::TimeSeriesQuery,
};
use crate::common::{
    core::tsdb::TSDB,
    data::{
        time_series_id::TimeSeriesID, time_series_data_source::TimeSeriesDataSource,
        time_series_data_source_config::TimeSeriesDataSourceConfig,
    },
    stats::span::Span,
};

pub(crate) trait QueryPipelineContext: QueryNode {
    fn factory(&self) -> Box<dyn QueryNodeFactory>;

    fn tsdb(&self) -> Box<dyn TSDB>;

    fn query(&self) -> Box<dyn TimeSeriesQuery>;

    fn query_context(&self) -> Box<dyn QueryContext>;

    fn initialize(&self, span: Box<dyn Span>);

    fn fetch_next(&self, span: Box<dyn Span>);

    fn upstream(&self, node: Box<dyn QueryNode>) -> Vec<Box<dyn QueryNode>>;

    fn downstream(&self, node: Box<dyn QueryNode>) -> Vec<Box<dyn QueryNode>>;

    fn downstream_sources(&self, node: Box<dyn QueryNode>) -> Vec<Box<dyn TimeSeriesDataSource>>;

    fn common_source_config(&self, node: Box<dyn QueryNode>)
    -> Box<dyn TimeSeriesDataSourceConfig>;

    fn downstream_sources_ids(&self, node: Box<dyn QueryNode>) -> Vec<String>;

    fn downstream_sources_result_ids(
        &self,
        node: Box<dyn QueryNode>,
    ) -> Vec<Box<dyn QueryResultID>>;

    fn upstream_of_type(&self, node: Box<dyn QueryNode>) -> Vec<Box<dyn QueryNode>>;

    fn downstream_of_type(&self, node: Box<dyn QueryNode>) -> Vec<Box<dyn QueryNode>>;

    fn sinks(&self) -> Vec<Box<dyn QuerySink>>;

    fn add_id(&self, hash: u64, id: Box<dyn TimeSeriesID>);

    fn get_id(&self, hash: u64) -> Box<dyn TimeSeriesID>;

    fn has_id(&self, hash: u64) -> bool;

    fn close(&self);
}
