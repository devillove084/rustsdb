use super::time_series_datatype::TimeSeriesDataType;

pub(crate) trait TypedTimeSeriesIterator<T: TimeSeriesDataType + Iterator> {
    fn get_type(&self) -> &str;
}

impl<T> Iterator for dyn TypedTimeSeriesIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
