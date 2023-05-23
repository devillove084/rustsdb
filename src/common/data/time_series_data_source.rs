use crate::common::query::{query_node::QueryNode, query_node_config::Builder};

use super::time_series_data_source_config::TimeSeriesDataSourceConfig;

// TODO: C is different from T
pub(crate) trait TimeSeriesDataSource<B, C>
where
    Self: QueryNode<B, C>,
    B: Builder<B, C>,
    C: TimeSeriesDataSourceConfig<B, C>,
{
}
