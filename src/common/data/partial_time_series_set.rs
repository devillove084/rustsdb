use crate::common::query::query_node::QueryNode;

use super::{time_specification::TimeSpecification, timestamp::TimeStamp};

#[async_trait::async_trait]
pub(crate) trait PartialTimeSeriesSet {
    fn total_sets(&self) -> i32;

    fn complete(&self) -> bool;

    // fn node(&self) -> impl QueryNode;

    fn data_source(&self) -> String;

    fn start(&self) -> Box<dyn TimeStamp>;

    fn end(&self) -> Box<dyn TimeStamp>;

    fn time_series_count(&self) -> i32;

    fn time_specification(&self) -> Box<dyn TimeSpecification>;
}
