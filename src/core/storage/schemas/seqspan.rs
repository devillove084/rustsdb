use super::rowseq::RowSeq;
use crate::common::{core::tsdb::TSDB, data::time_series_datatype::TimeSeriesDataType};

pub(crate) trait SeqSpan<T: TimeSeriesDataType> {
    fn add_sequence(&self, tsdb: Box<dyn TSDB>, sequence: Box<dyn RowSeq>, keep_earliest: bool);
}
