use super::{query_node::QueryNode, query_result_id::QueryResultID};
use crate::common::{
    data::{
        time_series::TimeSeries, time_series_id::TimeSeriesID,
        time_specification::TimeSpecification,
    },
    rollup::rollup_config::RollupConfig,
};

pub(crate) trait QueryResult {
    fn time_specification(&self) -> Box<dyn TimeSpecification>;

    fn time_series(&self) -> Vec<Box<dyn TimeSeries>>;

    fn error(&self) -> String;

    fn exception(&self);

    fn sequence_id(&self) -> u64;

    fn data_source(&self) -> Box<dyn QueryResultID>;

    fn id_type(&self) -> Box<dyn TimeSeriesID>;

    fn resolution(&self) -> u64;

    fn rollup_config(&self) -> Box<dyn RollupConfig>;

    fn close(&self);

    fn process_in_parallel(&self) -> bool;

    fn source(&self) -> Box<dyn QueryNode>;
}
