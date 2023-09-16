use bytes::Bytes;

use super::{
    time_series_byte_id::TimeSeriesByteID,
    time_series_data_source_config::TimeSeriesDataSourceConfig, time_series_id::TimeSeriesID,
    time_series_string_id::TimeSeriesStringID,
};
use crate::common::{
    query::{query_node_config::QueryNodeConfig, query_pipeline_context::QueryPipelineContext},
    rollup::rollup_config::RollupConfig,
    stats::span::Span,
};

#[async_trait::async_trait]
pub(crate) trait TimeSeriesDataSourceFactory {
    fn id_type(&self) -> Box<dyn TimeSeriesID>;

    fn supports_query(
        &self,
        context: Box<dyn QueryPipelineContext>,
        config: Box<dyn TimeSeriesDataSourceConfig>,
    ) -> bool;

    fn supports_push_down(&self, operation: Box<dyn QueryNodeConfig>) -> bool;

    async fn resolve_byte_id(
        &self,
        id: Box<dyn TimeSeriesByteID + Send>,
        span: Box<dyn Span + Send>,
    ) -> Box<dyn TimeSeriesStringID + Send>;

    async fn encode_join_keys(
        &self,
        join_keys: Vec<String>,
        span: Box<dyn Span + Send>,
    ) -> Vec<Vec<Bytes>>;

    async fn encode_join_metrics(
        &self,
        join_metrics: Vec<String>,
        span: Box<dyn Span + Send>,
    ) -> Vec<Vec<Bytes>>;

    fn rollup_config(&self) -> Box<dyn RollupConfig>;
}
