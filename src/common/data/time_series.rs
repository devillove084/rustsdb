use super::{
    time_series_byte_id::TimeSeriesID, time_series_datatype::TimeSeriesDataType,
    typed_time_series_iter::TypedTimeSeriesIterator,
};

/// A collection of data for a specific time series, i.e. {@link TimeSeriesId}.
/// The collection may contain multiple data types, e.g. a series of numeric
/// values and a series of strings or annotations.
/// <p>
/// To pull data out of this collection, call {@link #iterator(TypeToken)} or
/// {@link #iterators()}. These will return Java iterators that return
/// {@link TimeSeriesValue} objects on each call to {@link Iterator#next()}. Each
/// call to one of these iterator methods will return a new view of the data
/// starting at the beginning of the set.
/// <p>
/// All data types will contain data within the same time range for this time series.

pub(crate) trait TimeSeries<T: TimeSeriesID> {
    fn id(&self) -> T;

    fn iterator<IT: TimeSeriesDataType + Iterator>(
        &self,
        t: IT,
    ) -> Option<impl TypedTimeSeriesIterator<IT>>;
}
