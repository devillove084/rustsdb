use tokio::runtime::Runtime;

#[derive(Clone)]
pub(crate) struct ExecutorService {
    executor: &'static Runtime,
}
