use crate::common::core::tsdb::TSDB;

#[async_trait::async_trait]
pub(crate) trait Validatable {
    async fn validate(&self, tsdb: Box<dyn TSDB>);

    async fn validate_collection(
        &self,
        tsdb: Box<dyn TSDB>,
        collection: Vec<Box<dyn Validatable>>,
        name: String,
    ) -> Box<dyn Validatable>;

    async fn validate_pojo(
        &self,
        tsdb: Box<dyn TSDB>,
        pojo: Box<dyn Validatable>,
        name: String,
    ) -> Box<dyn Validatable>;
}
