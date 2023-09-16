use bytes::Bytes;

pub(crate) trait RollupInterval {
    fn get_table(&self) -> String;

    fn get_temporal_table(&self) -> Vec<Bytes>;

    fn get_pre_aggregation_table(&self) -> String;

    fn get_group_by_table(&self) -> Vec<Bytes>;

    fn get_interval(&self) -> String;

    fn get_units(&self) -> String;

    fn get_unit_multiplier(&self) -> i32;

    fn get_interval_units(&self) -> String;

    fn get_interval_seconds(&self) -> i32;

    fn get_interval_count(&self) -> i32;

    fn is_default_interval(&self) -> bool;

    fn get_row_span(&self) -> String;

    fn get_rollup_config(&self) -> Box<dyn RollupInterval>;

    fn set_rollup_config(&self, config: Box<dyn RollupInterval>);
}
