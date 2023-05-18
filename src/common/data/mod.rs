pub mod time_series;
pub mod time_series_byte_id;
pub mod time_series_datatype;
pub mod time_series_value;
pub mod timestamp;
pub mod typed_time_series_iter;

pub(crate) trait Comparable: PartialEq + PartialOrd + Eq + Ord {}
