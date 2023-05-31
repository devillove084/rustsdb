use std::collections::HashMap;

use crate::common::data::{
    time_series::TimeSeries, time_series_datatype::TimeSeriesDataType,
    typed_time_series_iter::TypedTimeSeriesIterator,
};

use super::{
    query_node::QueryNode,
    query_node_config::{Builder, QueryNodeConfig},
    query_result::QueryResult,
};

pub(crate) trait QueryIteratorFactory<B, C>
where
    B: Builder<B, C>,
    C: QueryNodeConfig<B, C>,
{
    fn types(&self) -> Vec<Box<dyn TimeSeriesDataType>>;

    fn new_iterator(
        &self,
        node: Box<dyn QueryNode<B, C>>,
        result: Box<dyn QueryResult>,
        sources: Vec<Box<dyn TimeSeries>>,
        typ: Box<dyn TimeSeriesDataType>,
    ) -> Box<dyn TypedTimeSeriesIterator<Box<dyn TimeSeriesDataType>>>;

    fn new_iterator_with_map(
        &self,
        node: Box<dyn QueryNode<B, C>>,
        result: Box<dyn QueryResult>,
        sources: HashMap<String, Box<dyn TimeSeries>>,
        typ: Box<dyn TimeSeriesDataType>,
    ) -> Box<dyn TypedTimeSeriesIterator<Box<dyn TimeSeriesDataType>>>;
}
