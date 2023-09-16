use std::collections::HashMap;

use multimap::MultiMap;

use super::{time_series_datatype::TimeSeriesDataType, timestamp::TimeStamp};

pub(crate) trait TimeSeriesShardTagsAndTimeData {
    fn timestamp(&self) -> Box<dyn TimeStamp>;

    fn tags(&self) -> HashMap<String, String>;

    fn data(&self) -> MultiMap<String, Box<dyn TimeSeriesDataType>>;

    fn size(&self) -> i32;
}
