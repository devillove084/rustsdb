use dyn_clone::{clone_trait_object, DynClone};

use super::pooled_object::PooledObject;

#[async_trait::async_trait]
pub(crate) trait ObjectPool: DynClone {
    async fn claim(&self) -> Box<dyn PooledObject>;

    async fn claim_by_time(&self, time: u64, unit: u64) -> Box<dyn PooledObject>;

    fn id(&self) -> String;

    async fn shutdown(&self);
}

clone_trait_object!(ObjectPool);
