use super::write_callback::WriteCallback;
use crate::common::data::{
    low_level_time_series_data::LowLevelTimeSeriesData, time_series_datum::TimeSeriesDatum,
    time_series_shared_tags_and_time_data::TimeSeriesShardTagsAndTimeData,
};

#[async_trait::async_trait]
pub(crate) trait TimeSeriesDataConsumer {
    async fn write(
        &self,
        datum: Box<dyn TimeSeriesDatum + Send>,
        callback: Box<dyn WriteCallback + Send>,
    );

    async fn write_with_shard_tags(
        &self,
        data: Box<dyn TimeSeriesShardTagsAndTimeData + Send>,
        callback: Box<dyn WriteCallback + Send>,
    );

    async fn write_with_low_level(
        &self,
        data: Box<dyn LowLevelTimeSeriesData + Send>,
        callback: Box<dyn WriteCallback + Send>,
    );
}
