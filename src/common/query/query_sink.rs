use crate::common::data::partial_time_series::PartialTimeSeries;

use super::{query_result::QueryResult, query_sink_callback::QuerySinkCallback};

pub(crate) trait QuerySink {
    fn on_complete(&self);

    fn on_next(&self, next: Box<dyn QueryResult>);

    fn on_next_with_callback(
        &self,
        next: Box<dyn PartialTimeSeries>,
        callback: Box<dyn QuerySinkCallback>,
    );

    fn on_error(&self);
}
