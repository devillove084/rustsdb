use super::source_node::SourceNode;
use crate::common::data::time_series_data_source::TimeSeriesDataSource;

pub(crate) trait TSDBQueryNode: TimeSeriesDataSource + SourceNode {
    fn set_sent_data(&self);

    fn sent_data(&self) -> bool;
}
