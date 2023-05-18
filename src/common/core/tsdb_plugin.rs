use crate::common::core::tsdb::TSDB;

#[async_trait::async_trait]
pub(crate) trait TSDBPlugin<T> {
    type PluginUseTSDB: TSDB<T>;

    fn typ(&self) -> String;

    fn id(&self) -> String;

    async fn initialize(&self, tsdb: Self::PluginUseTSDB, id: String);

    async fn shutdown(&self);

    fn version(&self) -> String;
}
