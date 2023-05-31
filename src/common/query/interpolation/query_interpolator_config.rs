use crate::common::data::time_series_datatype::TimeSeriesDataType;

pub(crate) trait QueryInterpolatorConfig {
    fn get_type(&self) -> String;

    fn get_data_type(&self) -> String;

    fn config_type(&self) -> Box<dyn TimeSeriesDataType>;
}
