use crate::common::{
    data::{time_series::TimeSeries, time_series_id::TimeSeriesID},
    query::{
        query_node::QueryNode,
        query_node_config::{Builder, QueryNodeConfig},
        query_result::QueryResult,
        query_result_id::QueryResultID,
    },
    rollup::rollup_config::RollupConfig,
};

pub(crate) struct BadQueryResult {
    node: Box<dyn QueryNode>,
    data_source: Box<dyn QueryResultID>,
    error: String,
}

impl QueryResult for BadQueryResult {
    fn time_specification(
        &self,
    ) -> Box<dyn crate::common::data::time_specification::TimeSpecification> {
        todo!()
    }

    fn time_series(&self) -> Vec<Box<dyn TimeSeries>> {
        todo!()
    }

    fn error(&self) -> String {
        todo!()
    }

    fn exception(&self) {
        todo!()
    }

    fn sequence_id(&self) -> u64 {
        todo!()
    }

    fn data_source(&self) -> Box<dyn QueryResultID> {
        todo!()
    }

    fn id_type(&self) -> Box<dyn TimeSeriesID> {
        todo!()
    }

    fn resolution(&self) -> u64 {
        todo!()
    }

    fn rollup_config(&self) -> Box<dyn RollupConfig> {
        todo!()
    }

    fn close(&self) {
        todo!()
    }

    fn process_in_parallel(&self) -> bool {
        todo!()
    }

    fn source(&self) -> Box<dyn QueryNode> {
        todo!()
    }
}
