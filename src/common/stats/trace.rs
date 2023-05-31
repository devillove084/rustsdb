use super::span::{Span, SpanBuilder};

#[async_trait::async_trait]
pub(crate) trait Trace {
    async fn new_span(&self, id: String) -> Box<dyn SpanBuilder>;

    async fn new_span_with_tags(&self, id: String, tags: Vec<String>) -> Box<dyn SpanBuilder>;

    async fn new_span_with_thread(&self, id: String) -> Box<dyn SpanBuilder>;

    async fn new_span_with_thread_and_tags(
        &self,
        id: String,
        tags: Vec<String>,
    ) -> Box<dyn SpanBuilder>;

    async fn is_debug(&self) -> bool;

    fn trace_id(&self) -> String;

    async fn first_span(&self) -> Box<dyn Span>;
}
