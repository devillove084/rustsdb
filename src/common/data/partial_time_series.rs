use super::{
    partial_time_series_set::PartialTimeSeriesSet, time_series_byte_id::TimeSeriesID,
    time_series_datatype::TimeSeriesDataType,
};

pub(crate) trait PartialTimeSeries: TimeSeriesDataType {
    fn id_hash(&self) -> u64;

    fn id_type(&self) -> Box<dyn TimeSeriesID>;

    fn set(&self) -> Box<dyn PartialTimeSeriesSet>;

    fn value(&self) -> Box<dyn TimeSeriesDataType>;
}
