use super::{time_series_datatype::TimeSeriesDataType, timestamp::TimeStamp, Comparable};

/// Represents a single object at a point in time of the given
/// `TimeSeriesDataType`. I. e. this is a single data point and the
/// `value` can be any data type supported by rustsdb.
///
/// Warning: Note that when a value is extracted from an iterator, a copy
/// should be made as the iterator may change the actual value on the next
/// iteration.
pub(crate) trait TimeSeriesValue<T: TimeSeriesDataType, C: Comparable> {
    type TimeSeriesValueComparator = C;

    fn timestamp<Ts: TimeStamp>(&self) -> Ts;

    fn value(&self) -> T;

    fn get_type(&self) -> &str;
}
