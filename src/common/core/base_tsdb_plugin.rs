use super::{tsdb::TSDB, tsdb_plugin::TSDBPlugin};

pub(crate) struct BaseTSDBPlugin {
    tsdb: Box<dyn TSDB>,

    id: String,
}

unsafe impl Send for BaseTSDBPlugin {}
unsafe impl Sync for BaseTSDBPlugin {}

#[async_trait::async_trait]
impl TSDBPlugin for BaseTSDBPlugin {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn typ(&self) -> String {
        todo!()
    }

    fn version(&self) -> String {
        "3.0.0".to_string()
    }

    async fn initialize(&mut self, tsdb: Box<dyn TSDB>, id: String) {
        self.tsdb = tsdb;
        self.id = id;
    }

    async fn shutdown(&self) {
        todo!()
    }
}
