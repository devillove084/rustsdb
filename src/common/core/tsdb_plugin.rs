use crate::common::core::tsdb::TSDB;

#[async_trait::async_trait]
pub(crate) trait TSDBPlugin {
    fn typ(&self) -> String;

    fn id(&self) -> String;

    async fn initialize(&self, tsdb: Box<dyn TSDB>, id: String);

    async fn shutdown(&self);

    fn version(&self) -> String;
}
