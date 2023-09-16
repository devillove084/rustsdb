use super::{
    partial_time_series_set::PartialTimeSeriesSet, time_series_datatype::TimeSeriesDataType,
    time_series_id::TimeSeriesID,
};

pub(crate) trait PartialTimeSeries: TimeSeriesDataType {
    fn id_hash(&self) -> u64;

    fn id_type(&self) -> Box<dyn TimeSeriesID>;

    fn set(&self) -> Box<dyn PartialTimeSeriesSet>;

    fn value(&self) -> Box<dyn TimeSeriesDataType>;
}
