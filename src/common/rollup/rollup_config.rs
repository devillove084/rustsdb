use std::collections::HashMap;

use bytes::Bytes;

use super::rollup_interval::RollupInterval;

pub(crate) trait RollupConfig {
    fn get_aggregation_ids(&self) -> HashMap<String, i32>;

    fn get_aggregation_for_id(&self, id: i32) -> String;

    fn get_id_for_aggregator(&self, aggregator: String) -> i32;

    fn get_intervals(&self) -> Vec<String>;

    fn get_possible_intervals(&self, interval: String) -> Vec<String>;

    fn get_rollup_intervals(
        &self,
        interval: u64,
        str_interval: String,
        skip_default: bool,
    ) -> Vec<Box<dyn RollupInterval>>;

    fn get_default_intervals(&self) -> Box<dyn RollupInterval>;

    fn get_id_for_aggregator_with_qualifier(&self, qualifier: Vec<Bytes>) -> i32;

    fn get_offset_start_from_qualifier(&self, qualifier: Vec<Bytes>) -> i32;
}
