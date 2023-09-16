use std::hash::Hash;

use super::time_series_id::TimeSeriesID;

pub(crate) trait TimeSeriesDatumID {
    fn get_type(&self) -> Box<dyn TimeSeriesID>;
}

impl PartialEq for Box<dyn TimeSeriesDatumID> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for Box<dyn TimeSeriesDatumID> {}

impl Hash for Box<dyn TimeSeriesDatumID> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}
