use std::collections::{HashMap, HashSet};

use bytes::Bytes;

use super::{
    time_series_data_source_factory::TimeSeriesDataSourceFactory, time_series_id::TimeSeriesID,
    time_series_string_id::TimeSeriesStringID,
};
use crate::common::stats::span::Span;

pub(crate) trait TimeSeriesByteID: TimeSeriesID {
    fn data_store(&self) -> Box<dyn TimeSeriesDataSourceFactory>;

    fn alias(&self) -> Vec<Bytes>;

    fn namespace(&self) -> Vec<Bytes>;

    fn metric(&self) -> Vec<Bytes>;

    fn tags(&self) -> HashMap<Vec<Bytes>, Vec<Bytes>>;

    fn aggregated_tags(&self) -> Vec<Vec<Bytes>>;

    fn disjoint_tags(&self) -> Vec<Vec<Bytes>>;

    fn unique_ids(&self) -> HashSet<Bytes>;

    fn skip_metric(&self) -> bool;

    fn decode(&self, cache: bool, span: Box<dyn Span>) -> Box<dyn TimeSeriesStringID>;
}
