use crate::common::{
    core::tsdb_plugin::TSDBPlugin, data::time_series_datatype::TimeSeriesDataType,
    pool::object_pool::ObjectPool,
};
use std::{collections::HashMap, future::Future};

use super::tsdb::TSDB;

#[async_trait::async_trait]
pub(crate) trait Registry<T> {
    type Object;
    type RegistryTSDB: TSDB<T>;
    type RegisterTimeSeriesType: TimeSeriesDataType;
    type RegistryObjectPool: ObjectPool;

    // TODO: How to represent a "defered<Object>"?
    async fn initialize(&self, load_plugins: bool);
    // TODO: may not be best choice
    async fn cleanup_pool(&self) -> dyn Future<Output = T>;

    async fn register_plugin<U>(
        &self,
        id: String,
        plugin: dyn TSDBPlugin<T, PluginUseTSDB = Self::RegistryTSDB>,
    );

    async fn get_default_plugin<U>(&self) -> U;

    async fn get_plugin<U>(&self, id: String) -> U;

    async fn get_plugins<U>(&self) -> Vec<U>;

    async fn plugins<U>(
        &self,
    ) -> HashMap<U, HashMap<String, Box<dyn TSDBPlugin<T, PluginUseTSDB = Self::RegistryTSDB>>>>;

    async fn register_shared_object(&self, id: String, obj: Self::Object) -> Self::Object;

    async fn register_object_pool(&self, pool: Self::RegistryObjectPool);

    fn get_object_pool(&self, id: String) -> Self::RegistryObjectPool;

    fn get_shared_object(&self, id: String) -> Self::Object;

    fn shared_object(&self) -> HashMap<String, Self::Object>;

    // TODO: need add query node factory

    async fn register_type(
        &self,
        typ: Self::RegisterTimeSeriesType,
        name: String,
        is_default_name: bool,
    );

    fn get_default_name(&self, tye: Self::RegisterTimeSeriesType) -> String;

    fn type_map(&self) -> HashMap<String, Self::RegisterTimeSeriesType>;

    fn default_type_name_map(&self) -> HashMap<Self::RegisterTimeSeriesType, String>;

    async fn shutdown(&self);
}
