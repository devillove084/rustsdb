use super::{
    batch_meta_query::{BatchMetaQuery, QueryType},
    meta_query::MetaQuery,
};
use crate::common::{
    core::tsdb::TSDB,
    data::time_series_data_source_config::TimeSeriesDataSourceConfig,
    query::{
        query_pipeline_context::QueryPipelineContext,
        util::{JsonNode, ObjectMapper},
    },
    stats::span::Span,
};

#[async_trait::async_trait]
pub(crate) trait MetaDataStorageSchema {
    async fn run_query(&self, query: Box<dyn BatchMetaQuery>, span: Box<dyn Span>);

    async fn run_query_with_context(
        &self,
        context: Box<dyn QueryPipelineContext>,
        config: Box<dyn TimeSeriesDataSourceConfig>,
        span: Box<dyn Span>,
    );

    async fn parse(
        &self,
        tsdb: Box<dyn TSDB>,
        mapper: ObjectMapper,
        jsonnode: JsonNode,
        query_type: QueryType,
    ) -> Box<dyn MetaQuery>;
}
