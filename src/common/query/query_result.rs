use crate::common::{
    data::{
        time_series::TimeSeries, time_series_byte_id::TimeSeriesID,
        time_specification::TimeSpecification,
    },
    rollup::rollup_config::RollupConfig,
};

use super::{query_node::QueryNode, query_result_id::QueryResultID};

#[async_trait::async_trait]
pub(crate) trait QueryResult {
    async fn time_specification(&self) -> Box<dyn TimeSpecification>;

    async fn time_series(&self) -> Vec<Box<dyn TimeSeries>>;

    async fn error(&self) -> String;

    async fn exception(&self);

    fn sequence_id(&self) -> u64;

    // async fn source<B, C>(&self) -> impl QueryNode<B, C>;

    async fn data_source(&self) -> Box<dyn QueryResultID>;

    async fn id_type(&self) -> Box<dyn TimeSeriesID>;

    async fn resolution(&self) -> u64;

    fn rollup_config(&self) -> Box<dyn RollupConfig>;

    async fn close(&self);

    fn process_in_parallel(&self) -> bool;
}
