use crate::common::rollup::rollup_config::RollupConfig;

pub(crate) struct DefaultRollupConfig {}

impl RollupConfig for DefaultRollupConfig {
    fn get_aggregation_ids(&self) -> std::collections::HashMap<String, i32> {
        todo!()
    }

    fn get_aggregation_for_id(&self, id: i32) -> String {
        todo!()
    }

    fn get_id_for_aggregator(&self, aggregator: String) -> i32 {
        todo!()
    }

    fn get_intervals(&self) -> Vec<String> {
        todo!()
    }

    fn get_possible_intervals(&self, interval: String) -> Vec<String> {
        todo!()
    }

    fn get_rollup_intervals(
        &self,
        interval: u64,
        str_interval: String,
        skip_default: bool,
    ) -> Vec<Box<dyn crate::common::rollup::rollup_interval::RollupInterval>> {
        todo!()
    }

    fn get_default_intervals(
        &self,
    ) -> Box<dyn crate::common::rollup::rollup_interval::RollupInterval> {
        todo!()
    }

    fn get_id_for_aggregator_with_qualifier(&self, qualifier: Vec<bytes::Bytes>) -> i32 {
        todo!()
    }

    fn get_offset_start_from_qualifier(&self, qualifier: Vec<bytes::Bytes>) -> i32 {
        todo!()
    }
}
