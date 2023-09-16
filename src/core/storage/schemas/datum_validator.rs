use crate::common::data::time_series_datum_id::TimeSeriesDatumID;

pub(crate) trait DatumIDValidator {
    fn validate(&self, id: Box<dyn TimeSeriesDatumID>) -> String;
}
