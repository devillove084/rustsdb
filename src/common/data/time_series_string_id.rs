use std::collections::{HashMap, HashSet};

use super::time_series_id::TimeSeriesID;

pub(crate) trait TimeSeriesStringID: TimeSeriesID {
    fn alias(&self) -> String;

    fn namespace(&self) -> String;

    fn metric(&self) -> String;

    fn tags(&self) -> HashMap<String, String>;

    fn get_tag_value(&self, key: String) -> String;

    fn aggregated_tags(&self) -> Vec<String>;

    fn disjoint_tags(&self) -> Vec<String>;

    fn unique_ids(&self) -> HashSet<String>;

    fn hits(&self) -> u64;
}
