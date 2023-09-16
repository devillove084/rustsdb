use bytes::Bytes;

use super::{rowseq::RowSeq, seqspan::SeqSpan};
use crate::common::{
    data::{time_series_datatype::TimeSeriesDataType, time_series_value::TimeSeriesValue},
    rollup::rollup_interval::RollupInterval,
    storage::write_status::WriteStatus,
};

pub(crate) trait Codec {
    fn get_type(&self) -> Box<dyn TimeSeriesDataType>;

    fn new_sequence(&self, reversed: bool) -> Box<dyn SeqSpan<Box<dyn TimeSeriesDataType>>>;

    fn new_rowseq(&self, base_time: u64) -> Box<dyn RowSeq>;

    fn encode(
        &self,
        value: Box<dyn TimeSeriesValue>,
        append_format: bool,
        base_time: i32,
        rollup_interval: Box<dyn RollupInterval>,
    ) -> Box<dyn WriteStatus>;

    fn reset(&self);

    fn encoded_values(&self) -> i32;

    fn qualifiers(&self) -> Vec<Vec<Bytes>>;

    fn values(&self) -> Vec<Vec<Bytes>>;

    fn qualifier_lengths(&self) -> Vec<i32>;

    fn value_lengths(&self) -> Vec<i32>;
}
