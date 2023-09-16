use bytes::Bytes;

use super::schema::Schema;
use crate::{
    common::{
        core::{base_tsdb_plugin::BaseTSDBPlugin, tsdb::TSDB, tsdb_plugin::TSDBPlugin},
        data::{
            time_series_byte_id::TimeSeriesByteID,
            time_series_data_consumer_factory::TimeSeriesDataConsumerFactory,
            time_series_data_source_config::TimeSeriesDataSourceConfig,
            time_series_data_source_factory::TimeSeriesDataSourceFactory,
            time_series_id::TimeSeriesID, time_series_string_id::TimeSeriesStringID,
        },
        query::{query_node_config::QueryNodeConfig, query_pipeline_context::QueryPipelineContext},
        rollup::rollup_config::RollupConfig,
        stats::span::Span,
        storage::time_series_data_consumer::TimeSeriesDataConsumer,
    },
    core::rollup_config::default_rollup_config::DefaultRollupConfig,
};

pub const SchemaFactoryType: &str = "Tsdb1xSchemaFactory";
pub const SchemaFactoryKeyPrefix: &str = "tsd.storage.";
pub const RollupEnableKey: &str = "rollups.enable";
pub const RollupKey: &str = "rollups.config";

pub(crate) struct SchemaFactory {
    schema: Box<Schema>,
    rollup_config: DefaultRollupConfig,
}

unsafe impl Send for SchemaFactory {}
unsafe impl Sync for SchemaFactory {}

#[async_trait::async_trait]
impl TSDBPlugin for SchemaFactory {
    fn id(&self) -> String {
        todo!()
    }

    fn get_type(&self) -> String {
        todo!()
    }

    fn version(&self) -> String {
        todo!()
    }

    async fn initialize(&mut self, tsdb: Box<dyn TSDB>, id: String) {
        todo!()
    }

    async fn shutdown(&self) {
        todo!()
    }
}

#[async_trait::async_trait]
impl TimeSeriesDataSourceFactory for SchemaFactory {
    fn id_type(&self) -> Box<dyn TimeSeriesID> {
        todo!()
    }

    fn supports_query(
        &self,
        context: Box<dyn QueryPipelineContext>,
        config: Box<dyn TimeSeriesDataSourceConfig>,
    ) -> bool {
        todo!()
    }

    fn supports_push_down(&self, operation: Box<dyn QueryNodeConfig>) -> bool {
        todo!()
    }

    async fn resolve_byte_id(
        &self,
        id: Box<dyn TimeSeriesByteID + Send>,
        span: Box<dyn Span + Send>,
    ) -> Box<dyn TimeSeriesStringID + Send> {
        todo!()
    }

    async fn encode_join_keys(
        &self,
        join_keys: Vec<String>,
        span: Box<dyn Span + Send>,
    ) -> Vec<Vec<Bytes>> {
        todo!()
    }

    async fn encode_join_metrics(
        &self,
        join_metrics: Vec<String>,
        span: Box<dyn Span + Send>,
    ) -> Vec<Vec<Bytes>> {
        todo!()
    }

    fn rollup_config(&self) -> Box<dyn RollupConfig> {
        todo!()
    }
}

impl TimeSeriesDataConsumerFactory for SchemaFactory {
    fn consumer(&self) -> Box<dyn TimeSeriesDataConsumer> {
        todo!()
    }
}
