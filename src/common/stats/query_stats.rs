use crate::common::{query::query_context::QueryContext, stats::trace::Trace};

use super::span::Span;

#[async_trait::async_trait]
pub(crate) trait QueryStats {
    async fn trace(&self) -> Box<dyn Trace>;

    async fn query_span(&self) -> Box<dyn Span>;

    async fn set_query_context(&self, context: Box<dyn QueryContext>);

    fn emit_stats(&self);

    fn increment_raw_datasize(&self, size: u64);

    fn increment_serialized_datasize(&self, size: u64);

    fn increment_raw_time_series_count(&self, count: u64);

    fn increment_serialized_time_series_count(&self, count: u64);

    fn raw_data_size(&self) -> u64;

    fn serialized_data_size(&self) -> u64;

    fn raw_time_series_count(&self) -> u64;

    fn serialized_time_series_count(&self) -> u64;
}
