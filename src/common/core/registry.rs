use crate::common::{
    core::tsdb_plugin::TSDBPlugin, data::time_series_datatype::TimeSeriesDataType,
    pool::object_pool::ObjectPool,
};
use std::{collections::HashMap, future::Future};

use super::tsdb::TSDB;

#[async_trait::async_trait]
pub(crate) trait Registry {
    // type Object;
    // type RegistryTSDB: TSDB;
    // type RegisterTimeSeriesType: TimeSeriesDataType;
    // type RegistryObjectPool: ObjectPool;

    async fn initialize(&self, load_plugins: bool);

    // async fn cleanup_pool(&self) -> dyn Future<Output = Self>;

    async fn register_plugin(&self, id: String, plugin: dyn TSDBPlugin);

    // async fn get_default_plugin<U>(&self) -> Box<U>;

    // async fn get_plugin<U>(&self, id: String) -> Box<U>;

    // async fn get_plugins<U>(&self) -> Vec<Box<U>>;

    // async fn plugins<U>(&self) -> HashMap<Box<U>, HashMap<String, Box<dyn TSDBPlugin>>>;

    // TODO: Box<()> represent the java's Object
    async fn register_shared_object(&self, id: String, obj: Box<()>) -> Box<()>;

    fn get_shared_object(&self, id: String) -> Box<()>;

    fn shared_object(&self) -> HashMap<String, Box<()>>;

    async fn register_object_pool(&self, pool: Box<dyn ObjectPool>);

    fn get_object_pool(&self, id: String) -> Box<dyn ObjectPool>;

    // TODO: need add query node factory

    async fn register_type(
        &self,
        typ: Box<dyn TimeSeriesDataType>,
        name: String,
        is_default_name: bool,
    );

    fn get_default_name(&self, tye: Box<dyn TimeSeriesDataType>) -> String;

    fn type_map(&self) -> HashMap<String, Box<dyn TimeSeriesDataType>>;

    fn default_type_name_map(&self) -> HashMap<Box<dyn TimeSeriesDataType>, String>;

    async fn shutdown(&self);
}
