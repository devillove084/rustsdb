use std::collections::HashMap;

use super::plugin_config::PluginsConfig;
use crate::{
    common::{
        core::{
            registry::{Registry, RegistryGetQueryOpt},
            tsdb::TSDB,
            tsdb_plugin::TSDBPlugin,
        },
        data::time_series_datatype::TimeSeriesDataType,
        pool::{executor::ExecutorService, object_pool::ObjectPool},
        query::{
            interpolation::query_interpolator_factory::QueryInterpolatorFactory,
            query_iter_factory::QueryIteratorFactory, query_node_factory::QueryNodeFactory,
            serdes::time_series_serdes::TimeSeriesSerdes,
        },
    },
    core::{
        pool::shared_obj::SharedObject,
        query::{
            execution::query_executor::QueryExecutorFactory,
            hacluster::ha_cluster_config::HAClusterConfig,
        },
    },
};

pub const PLUGIN_CONFIG_KEY: &str = "tsd.plugin.config";
pub const V2_LOAD_FILTERS_KEY: &str = "tsd.plugin.v2.load_filters";
pub const DEFAULT_CLUSTERS_KEY: &str = "tsd.query.default_clusters";
pub const DEFAULT_GRAPHS_KEY: &str = "tsd.query.default_execution_graphs";

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct DefaultRegistry {
    tsdb: Box<dyn TSDB>,
    type_map: HashMap<String, Box<dyn TimeSeriesDataType>>,
    default_type_name_map: HashMap<Box<dyn TimeSeriesDataType>, String>,
    factories: HashMap<String, Box<dyn QueryExecutorFactory>>,
    clusters: HashMap<String, HAClusterConfig>,
    serdes: HashMap<String, Box<dyn TimeSeriesSerdes>>,
    node_factories: HashMap<String, Box<dyn QueryNodeFactory>>,
    shared_objects: HashMap<String, SharedObject>,
    pools: HashMap<String, Box<dyn ObjectPool>>,
    cleanup_pool: ExecutorService,
    plugins: PluginsConfig,
}

unsafe impl Send for DefaultRegistry {}

unsafe impl Sync for DefaultRegistry {}

#[async_trait::async_trait]
impl Registry for DefaultRegistry {
    async fn initialize(&self, _load_plugins: bool) {
        todo!()
    }

    async fn cleanup_pool(&self) -> ExecutorService {
        todo!()
    }

    async fn register_plugin(&self, _id: String, _plugin: Box<dyn TSDBPlugin + Send>) {
        todo!()
    }

    fn get_default_plugin(&self) -> Box<dyn TSDBPlugin> {
        todo!()
    }

    fn get_plugin(&self, _id: String) -> Box<dyn TSDBPlugin> {
        todo!()
    }

    fn get_plugins(&self) -> Vec<Box<dyn TSDBPlugin>> {
        todo!()
    }

    fn get_plugins_with_id(&self) -> Vec<HashMap<String, Box<dyn TSDBPlugin>>> {
        todo!()
    }

    async fn register_shared_object(&self, _id: String, _obj: SharedObject) -> SharedObject {
        todo!()
    }

    fn get_shared_object(&self, _id: String) -> SharedObject {
        todo!()
    }

    fn shared_object(&self) -> HashMap<String, SharedObject> {
        todo!()
    }

    fn register_object_pool(&self, _pool: Box<dyn ObjectPool>) {
        todo!()
    }

    fn get_object_pool(&self, _id: String) -> Box<dyn ObjectPool> {
        todo!()
    }

    fn register_type(
        &self,
        _typ: Box<dyn TimeSeriesDataType>,
        _name: String,
        _is_default_name: bool,
    ) {
        todo!()
    }

    fn get_type(&self, _name: String) -> Box<dyn TimeSeriesDataType> {
        todo!()
    }

    fn get_default_type_name(&self, _typ: Box<dyn TimeSeriesDataType>) -> String {
        todo!()
    }

    fn type_map(&self) -> HashMap<String, Box<dyn TimeSeriesDataType>> {
        todo!()
    }

    fn default_type_name_map(&self) -> HashMap<Box<dyn TimeSeriesDataType>, String> {
        todo!()
    }

    async fn shutdown(&self) {
        todo!()
    }
}

impl RegistryGetQueryOpt for DefaultRegistry {
    fn get_query_iter_factory(&self, _id: String) -> Box<dyn QueryIteratorFactory> {
        todo!()
    }

    fn get_query_iter_interpolator_factory(
        &self,
        _id: String,
    ) -> Box<dyn QueryInterpolatorFactory> {
        todo!()
    }

    fn get_query_node_factory(&self, _id: String) -> Box<dyn QueryNodeFactory> {
        todo!()
    }
}
