use crate::common::{core::tsdb::TSDB, data::time_series_byte_id::TimeSeriesID};

use super::query_sink::QuerySink;

#[async_trait::async_trait]
pub(crate) trait QueryContext {
    async fn sinks(&self) -> Vec<Box<dyn QuerySink>>;

    // async fn mode(&self) -> QueryMode;

    // async fn fetch_next(&self, span: Span);

    // async fn close(&self);

    // fn is_closed(&self) -> bool;

    // async fn stats(&self) -> QueryStats;

    // async fn sink_configs(&self) -> Vec<QuerySinkConfig>;

    // async fn query(&self) -> TimeSeriesQuery;

    // async fn tsdb(&self) -> impl TSDB<T>;

    // async fn auth_state(&self) -> AuthState;

    // async fn headers(&self) -> bool;

    // fn cacheable(&self) -> bool;

    // async fn initialize(&self, span: Span);

    // fn get_id(&self, hash: u64, tye: Self::QueryTimeSeriesID) -> Self::QueryTimeSeriesID;

    // TODO: log opteartions!!
}
