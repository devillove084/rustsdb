#[async_trait::async_trait]
pub(crate) trait Span {
    //type Object;

    async fn finish(&self);

    async fn finish_with_timeout(&self, duration: u64);

    async fn set_success_tags(&self) -> Box<dyn Span>;

    async fn set_error_tags(&self) -> Box<dyn Span>;

    async fn set_tag(&self, key: String, value: String) -> Box<dyn Span>;

    async fn log(&self, key: String) -> Box<dyn Span>;

    // TODO: Box<()> means java's Object
    async fn implementation_span(&self) -> Box<()>;

    fn is_debug(&self) -> bool;

    async fn new_child(&self, id: String) -> Box<dyn SpanBuilder>;
}

#[async_trait::async_trait]
pub(crate) trait SpanBuilder {
    async fn as_child_of(&self, parent: Box<dyn Span>) -> Box<dyn SpanBuilder>;

    async fn build_span(&self, id: String) -> Box<dyn SpanBuilder>;

    async fn with_tag(&self, key: String, value: String) -> Box<dyn SpanBuilder>;

    async fn start(&self) -> Box<dyn Span>;
}
