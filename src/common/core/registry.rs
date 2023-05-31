use dyn_clone::{clone_trait_object, DynClone};

use crate::{
    common::{
        core::tsdb_plugin::TSDBPlugin,
        data::time_series_datatype::TimeSeriesDataType,
        pool::{executor::ExecutorService, object_pool::ObjectPool},
        query::{
            interpolation::query_interpolator_factory::QueryInterpolatorFactory,
            query_iter_factory::QueryIteratorFactory,
            query_node_config::{Builder, QueryNodeConfig},
            query_node_factory::QueryNodeFactory,
        },
    },
    core::pool::shared_obj::SharedObject,
};
use std::collections::HashMap;

#[async_trait::async_trait]
pub(crate) trait Registry: DynClone {
    async fn initialize(&self, load_plugins: bool);

    async fn cleanup_pool(&self) -> ExecutorService;

    async fn register_plugin(&self, id: String, plugin: Box<dyn TSDBPlugin>);

    fn get_default_plugin(&self) -> Box<dyn TSDBPlugin>;

    fn get_plugin(&self, id: String) -> Box<dyn TSDBPlugin>;

    fn get_plugins(&self) -> Vec<Box<dyn TSDBPlugin>>;

    fn get_plugins_with_id(&self) -> Vec<HashMap<String, Box<dyn TSDBPlugin>>>;

    async fn register_shared_object(&self, id: String, obj: SharedObject) -> SharedObject;

    fn get_shared_object(&self, id: String) -> SharedObject;

    fn shared_object(&self) -> HashMap<String, SharedObject>;

    fn register_object_pool(&self, pool: Box<dyn ObjectPool>);

    fn get_object_pool(&self, id: String) -> Box<dyn ObjectPool>;

    fn register_type(&self, typ: Box<dyn TimeSeriesDataType>, name: String, is_default_name: bool);

    fn get_type(&self, name: String) -> Box<dyn TimeSeriesDataType>;

    fn get_default_type_name(&self, typ: Box<dyn TimeSeriesDataType>) -> String;

    fn type_map(&self) -> HashMap<String, Box<dyn TimeSeriesDataType>>;

    fn default_type_name_map(&self) -> HashMap<Box<dyn TimeSeriesDataType>, String>;

    async fn shutdown(&self);
}

clone_trait_object!(Registry);

pub(crate) trait RegistryGetQueryOpt<B, C>
where
    B: Builder<B, C>,
    C: QueryNodeConfig<B, C>,
{
    fn get_query_node_factory(&self, id: String) -> Box<dyn QueryNodeFactory<B, C>>;

    fn get_query_iter_factory(&self, id: String) -> Box<dyn QueryIteratorFactory<B, C>>;

    fn get_query_iter_interpolator_factory(&self, id: String) -> Box<dyn QueryInterpolatorFactory>;
}
