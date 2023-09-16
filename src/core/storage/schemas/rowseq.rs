use bytes::Bytes;

use crate::common::{core::tsdb::TSDB, data::time_series_datatype::TimeSeriesDataType};

pub(crate) trait RowSeq {
    fn get_type(&self) -> Box<dyn TimeSeriesDataType>;

    fn add_column(&self, prefix: Bytes, qualifier: Vec<Bytes>, value: Vec<Bytes>);

    fn dedupe(&self, tsdb: Vec<Box<dyn TSDB>>, keep_earliest: bool, reverse: bool) -> u64;

    fn size(&self) -> i32;

    fn data_point(&self) -> i32;
}
