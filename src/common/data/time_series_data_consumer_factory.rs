use crate::common::{
    core::tsdb_plugin::TSDBPlugin, storage::time_series_data_consumer::TimeSeriesDataConsumer,
};

pub(crate) trait TimeSeriesDataConsumerFactory: TSDBPlugin {
    fn consumer(&self) -> Box<dyn TimeSeriesDataConsumer>;
}
