use crate::common::{
    data::{
        time_series::TimeSeries, time_series_byte_id::TimeSeriesID,
        time_specification::TimeSpecification,
    },
    rollup::rollup_config::RollupConfig,
};

use super::{
    query_node::QueryNode,
    query_node_config::{Builder, QueryNodeConfig},
    query_result_id::QueryResultID,
};

#[async_trait::async_trait]
pub(crate) trait GetSource<B, C>
where
    B: Builder<B, C>,
    C: QueryNodeConfig<B, C>,
{
    async fn source(&self) -> Box<dyn QueryNode<B, C>>;
}

#[async_trait::async_trait]
pub(crate) trait QueryResult {
    async fn time_specification(&self) -> Box<dyn TimeSpecification>;

    async fn time_series(&self) -> Vec<Box<dyn TimeSeries>>;

    async fn error(&self) -> String;

    async fn exception(&self);

    fn sequence_id(&self) -> u64;

    async fn data_source(&self) -> Box<dyn QueryResultID>;

    async fn id_type(&self) -> Box<dyn TimeSeriesID>;

    async fn resolution(&self) -> u64;

    fn rollup_config(&self) -> Box<dyn RollupConfig>;

    async fn close(&self);

    fn process_in_parallel(&self) -> bool;
}
