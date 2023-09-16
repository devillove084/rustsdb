use super::{
    time_series_datatype::TimeSeriesDataType, time_series_datum_id::TimeSeriesDatumID,
    time_series_value::TimeSeriesValue,
};

pub(crate) trait TimeSeriesDatum {
    fn id(&self) -> Box<dyn TimeSeriesDatumID>;

    fn value(&self) -> Box<dyn TimeSeriesValue>;
}

pub(crate) fn wrap(
    id: Box<dyn TimeSeriesDatumID>,
    value: Box<dyn TimeSeriesValue>,
) -> Box<dyn TimeSeriesDatum> {
    todo!()
}
