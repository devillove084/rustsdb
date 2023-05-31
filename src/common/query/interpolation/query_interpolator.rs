use crate::common::{
    data::{
        time_series_datatype::TimeSeriesDataType, time_series_value::TimeSeriesValue,
        timestamp::TimeStamp,
    },
    query::query_fill_policy::QueryFillPolicy,
};

pub(crate) trait QueryInterpolator {
    fn hash_next(&self) -> bool;

    fn next(&self, ts: Box<dyn TimeStamp>) -> Box<dyn TimeSeriesValue>;

    fn next_real(&self) -> Box<dyn TimeStamp>;

    fn fill_policy(&self) -> Box<dyn QueryFillPolicy>;
}
