use std::{any::Any, collections::HashMap};

use super::{
    codec::Codec, datum_validator::DatumIDValidator, schema_factory::SchemaFactory,
    tsdb_data_store::TSDBDataStore,
};
use crate::{
    common::{
        core::tsdb::TSDB,
        data::{
            low_level_time_series_data::LowLevelTimeSeriesData,
            partial_time_series::PartialTimeSeries, partial_time_series_set::PartialTimeSeriesSet,
            time_series_byte_id::TimeSeriesByteID, time_series_datatype::TimeSeriesDataType,
            time_series_datum::TimeSeriesDatum, time_series_id::TimeSeriesID,
            time_series_shared_tags_and_time_data::TimeSeriesShardTagsAndTimeData,
            time_series_string_id::TimeSeriesStringID, timestamp::TimeStamp,
        },
        pool::object_pool::ObjectPool,
        rollup::rollup_interval::RollupInterval,
        stats::span::Span,
        storage::{
            time_series_data_consumer::TimeSeriesDataConsumer, write_callback::WriteCallback,
        },
    },
    core::{
        meta::meta_data_storage_schema::MetaDataStorageSchema,
        uid::{uniqueid::UniqueID, uniqueid_store::UniqueIDStore},
    },
};

pub const APPENDS_PREFIX: i32 = 5;
pub const QUERY_BYTE_LIMIT_KEY: &str = "tsd.query.limits.bytes";
pub const QUERY_BYTE_LIMIT_DEFAULT: u64 = 0;
pub const QUERY_DP_LIMIT_KEY: &str = "tsd.query.limits.data_points";
pub const QUERY_DP_LIMIT_DEFAULT: u64 = 0;
pub const QUERY_REVERSE_KEY: &str = "tsd.query.time.descending";
pub const QUERY_KEEP_FIRST_KEY: &str = "tsd.query.duplicates.keep_earliest";
pub const TIMELESS_SALTING_KEY: &str = "tsd.storage.salt.timeless";
pub const OLD_SALTING_KEY: &str = "tsd.storage.salt.old";
pub const MAX_RAW_TIMESPAN: i16 = 3600;
pub const TIMESTAMP_BYTES: i16 = 4;
pub const METRIC_TYPE: &str = "metric";
pub const TAGK_TYPE: &str = "tagk";
pub const TAGV_TYPE: &str = "tagv";

pub(crate) struct Schema {
    tsdb: Box<dyn TSDB>,
    id: String,
    data_store: Box<dyn TSDBDataStore>,
    uid_store: Box<dyn UniqueIDStore>,
    metrics: Box<dyn UniqueID>,
    tag_names: Box<dyn UniqueID>,
    tag_values: Box<dyn UniqueID>,
    metric_width: i32,
    tagk_width: i32,
    tagv_width: i32,
    salt_buckets: i32,
    salt_width: i32,
    timeless_salting: bool,
    old_salting: bool,
    pool_cache: HashMap<Box<dyn Any>, Box<dyn ObjectPool>>,
    // TODO: Any represent TypeToken<?>, maybe better idea?
    codecs: HashMap<Box<dyn Any>, Box<dyn Codec>>,
    encoders: HashMap<Box<dyn Any>, Box<dyn Codec>>,
    meta_schema: Box<dyn MetaDataStorageSchema>,
    id_validator: Box<dyn DatumIDValidator>,
    factory: SchemaFactory,
}

unsafe impl Send for Schema {}
unsafe impl Sync for Schema {}

#[async_trait::async_trait]
impl TimeSeriesDataConsumer for Schema {
    async fn write(
        &self,
        datum: Box<dyn TimeSeriesDatum + Send>,
        callback: Box<dyn WriteCallback + Send>,
    ) {
        todo!()
    }

    async fn write_with_shard_tags(
        &self,
        data: Box<dyn TimeSeriesShardTagsAndTimeData + Send>,
        callback: Box<dyn WriteCallback + Send>,
    ) {
        todo!()
    }

    async fn write_with_low_level(
        &self,
        data: Box<dyn LowLevelTimeSeriesData + Send>,
        callback: Box<dyn WriteCallback + Send>,
    ) {
        todo!()
    }
}

impl Schema {
    pub fn config_key(&self, suffix: &str) -> String {
        "tsd.storage.".to_owned() + &self.id.to_string() + "." + &suffix
    }

    pub fn set_config(&mut self) {
        let key = self.config_key("uid.width.metric");
        if !self.tsdb.get_config().has_property(key.clone()) {
            self.tsdb.get_config().register_by_int_value(
                key.clone(),
                3,
                false,
                "The width, in bytes, of UIDs for metrics.".to_string(),
            )
        }
        self.metric_width = self.tsdb.get_config().get_int(key);

        let key = self.config_key("uid.width.tagk");
        if !self.tsdb.get_config().has_property(key.clone()) {
            self.tsdb.get_config().register_by_int_value(
                key.clone(),
                3,
                false,
                "The width, in bytes, of UIDs for tag keys.".to_string(),
            )
        }
        self.tagk_width = self.tsdb.get_config().get_int(key);

        let key = self.config_key("uid.width.tagv");
        if !self.tsdb.get_config().has_property(key.clone()) {
            self.tsdb.get_config().register_by_int_value(
                key.clone(),
                3,
                false,
                "The width, in bytes, of UIDs for tag values.".to_string(),
            )
        }
        self.tagv_width = self.tsdb.get_config().get_int(key);

        let key = self.config_key("uid.cache.type.metric");
        if !self.tsdb.get_config().has_property(key.clone()) {
            self.tsdb.get_config().register_by_string_value(
                key.clone(),
                "LRU".to_string(),
                false,
                "The name of the UniqueId factory used for caching metric UIDs.".to_string(),
            )
        }

        let key = self.config_key("uid.cache.type.tagk");
        if !self.tsdb.get_config().has_property(key.clone()) {
            self.tsdb.get_config().register_by_string_value(
                key.clone(),
                "LRU".to_string(),
                false,
                "The name of the UniqueId factory used for caching tagk UIDs.".to_string(),
            )
        }

        let key = self.config_key("uid.cache.type.tagv");
        if !self.tsdb.get_config().has_property(key.clone()) {
            self.tsdb.get_config().register_by_string_value(
                key.clone(),
                "LRU".to_string(),
                false,
                "The name of the UniqueId factory used for caching tagv UIDs.".to_string(),
            )
        }

        let key = self.config_key("salt.buckets");
        if !self.tsdb.get_config().has_property(key.clone()) {
            self.tsdb.get_config().register_by_int_value(
                key.clone(),
                20,
                false,
                "The number of salt buckets to spread data into.".to_string(),
            )
        }
        self.salt_buckets = self.tsdb.get_config().get_int(key);

        let key = self.config_key("salt.width");
        if !self.tsdb.get_config().has_property(key.clone()) {
            self.tsdb.get_config().register_by_int_value(
                key.clone(),
                0,
                false,
                "The width, in bytes, of the salt prefix in row keys.".to_string(),
            )
        }
        self.salt_width = self.tsdb.get_config().get_int(key);

        let key = self.config_key("data.store");
        if !self.tsdb.get_config().has_property(key.clone()) {
            self.tsdb.get_config().register_by_string_value(
                key.clone(),
                "".to_string(),
                false,
                "The name of the data store factory to load and associate with this schema."
                    .to_string(),
            )
        }

        let key = self.config_key("rollups.enable");
        if !self.tsdb.get_config().has_property(key.clone()) {
            self.tsdb.get_config().register_by_bool_value(
                key.clone(),
                false,
                false,
                "Whether or not rollups are enabled for this schema.".to_string(),
            )
        }

        let key = self.config_key("rollups.config");
        if !self.tsdb.get_config().has_property(key.clone()) {
            self.tsdb.get_config().register_by_string_value(
                key.clone(),
                "".to_string(),
                false,
                "The path to a JSON file containing the rollup configuration.".to_string(),
            )
        }
    }

    pub fn get_data_store(&self) -> Box<dyn TSDBDataStore> {
        self.data_store.clone()
    }

    pub fn get_uid_store(&self) -> Box<dyn UniqueIDStore> {
        self.uid_store.clone()
    }

    pub fn get_metrics(&self) -> Box<dyn UniqueID> {
        self.metrics.clone()
    }

    pub fn get_tag_names(&self) -> Box<dyn UniqueID> {
        self.tag_names.clone()
    }

    pub fn get_tag_values(&self) -> Box<dyn UniqueID> {
        self.tag_values.clone()
    }

    pub fn get_tsdb(&self) -> Box<dyn TSDB> {
        self.tsdb.clone()
    }

    async fn resolve_byte_id(
        &self,
        id: Box<dyn TimeSeriesByteID>,
        span: Box<dyn Span>,
    ) -> Box<dyn TimeSeriesStringID> {
        todo!()
    }

    pub fn new_series(
        &self,
        dtype: Box<dyn TimeSeriesDataType>,
        base_timestamp: Box<dyn TimeStamp>,
        id_hash: u64,
        set: Box<dyn PartialTimeSeriesSet>,
        interval: Box<dyn RollupInterval>,
    ) -> Box<dyn PartialTimeSeries> {
        todo!()
    }

    pub fn new_schema(&self, factory: SchemaFactory, tsdb: Box<dyn TSDB>, id: String) {
        todo!()
    }
}

struct MetricAndTagsDatumID {
    metric: String,
    tags: HashMap<String, String>,
}
impl TimeSeriesID for MetricAndTagsDatumID {
    fn encode(&self) -> bool {
        todo!()
    }

    fn hashcode(&self) -> u64 {
        todo!()
    }
}

impl TimeSeriesStringID for MetricAndTagsDatumID {
    fn alias(&self) -> String {
        todo!()
    }

    fn namespace(&self) -> String {
        todo!()
    }

    fn metric(&self) -> String {
        todo!()
    }

    fn tags(&self) -> HashMap<String, String> {
        todo!()
    }

    fn get_tag_value(&self, key: String) -> String {
        todo!()
    }

    fn aggregated_tags(&self) -> Vec<String> {
        todo!()
    }

    fn disjoint_tags(&self) -> Vec<String> {
        todo!()
    }

    fn unique_ids(&self) -> std::collections::HashSet<String> {
        todo!()
    }

    fn hits(&self) -> u64 {
        todo!()
    }
}
