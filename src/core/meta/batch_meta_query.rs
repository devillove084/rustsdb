use super::meta_query::MetaQuery;
use crate::common::data::timestamp::TimeStamp;

pub(crate) enum QueryType {
    NAMESPACES,
    METRICS,
    TAGKEYS,
    TAGVALUES,
    TAGKEYSANDVALUES,
    TIMESERIES,
    BASIC,
}

pub(crate) enum QueryOrder {
    ASCENDING,
    DESCENDING,
}

pub(crate) trait BatchMetaQuery {
    fn get_from(&self) -> i32;

    fn get_to(&self) -> i32;

    fn get_aggregation_field(&self) -> String;

    fn get_aggregation_size(&self) -> i32;

    fn get_query_type(&self) -> QueryType;

    fn get_query_order(&self) -> QueryOrder;

    fn get_start(&self) -> Box<dyn TimeStamp>;

    fn get_end(&self) -> Box<dyn TimeStamp>;

    fn get_queries(&self) -> Vec<Box<dyn MetaQuery>>;

    fn meta_query(&self) -> Vec<Box<dyn MetaQuery>>;

    fn source(&self) -> String;
}

pub(crate) struct BatchMetaQueryBuilder {
    from: i32,
    to: i32,
    aggregation_field: String,
    agg_size: i32,
    query_type: QueryType,
    source: String,
    query_order: QueryOrder,
    start: String,
    end: String,
    time_zone: String,
    meta_query: Vec<Box<dyn MetaQuery>>,
}

impl BatchMetaQueryBuilder {
    pub fn set_from(&mut self, from: i32) {
        self.from = from
    }

    pub fn set_aggregation_field(&mut self, agg_field: String) {
        self.aggregation_field = agg_field
    }

    pub fn set_aggregation_size(&mut self, size: i32) {
        self.agg_size = size
    }

    pub fn set_query_type(&mut self, query_type: QueryType) {
        self.query_type = query_type
    }

    pub fn set_source(&mut self, source: String) {
        self.source = source
    }

    pub fn set_query_order(&mut self, order: QueryOrder) {
        self.query_order = order
    }

    pub fn set_start(&mut self, start: String) {
        self.start = start
    }

    pub fn set_end(&mut self, end: String) {
        self.end = end
    }

    pub fn set_time_zone(&mut self, time_zone: String) {
        self.time_zone = time_zone
    }

    pub fn set_meta_query(&mut self, meta_query: Vec<Box<dyn MetaQuery>>) {
        self.meta_query = meta_query
    }

    pub fn build(&self) {
        todo!()
    }
}
