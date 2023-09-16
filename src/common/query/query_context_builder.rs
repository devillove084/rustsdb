use std::collections::HashMap;

use super::{
    query_context::QueryContext, query_mode::QueryMode, query_sink::QuerySink,
    query_sink_config::QuerySinkConfig, time_series_query::TimeSeriesQuery,
};
use crate::common::stats::query_stats::QueryStats;

pub(crate) trait QueryContextBuilder {
    fn set_query(&mut self, query: Box<dyn TimeSeriesQuery>);

    fn set_mode(&mut self, mode: QueryMode);

    fn set_stats(&mut self, stats: Box<dyn QueryStats>);

    fn set_sinks(&mut self, configs: Vec<Box<dyn QuerySinkConfig>>);

    fn add_sink(&mut self, sink: Box<dyn QuerySink>);

    fn set_local_sinks(&mut self, sinks: Vec<Box<dyn QuerySink>>);

    // fn set_auth_state(&mut self, auth_state: AuthState);

    fn set_header(&mut self, headers: HashMap<String, String>);

    fn build(&self) -> Box<dyn QueryContext>;
}
