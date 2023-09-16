use super::write_status::WriteStatus;

#[async_trait::async_trait]
pub(crate) trait WriteCallback {
    async fn success(&self);

    async fn partial_success(&self, status: Vec<Box<dyn WriteStatus>>, length: i32);

    async fn retry_all(&self);

    async fn fail_all(&self, status: Box<dyn WriteStatus>);
}
