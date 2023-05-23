use crate::common::query::query_node_config::{Builder, QueryNodeConfig};

pub(crate) trait TimeSeriesDataSourceConfig<B, C>
where
    Self: QueryNodeConfig<B, C>,
    B: Builder<B, C>,
    C: TimeSeriesDataSourceConfig<B, C>,
{
}
