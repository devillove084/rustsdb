use super::pooled_object::PooledObject;

#[async_trait::async_trait]
pub(crate) trait ObjectPool {
    async fn claim(&self) -> Box<dyn PooledObject>;

    async fn claim_by_time(&self, time: u64, unit: u64) -> Box<dyn PooledObject>;

    fn id(&self) -> String;

    async fn shutdown(&self);
}
