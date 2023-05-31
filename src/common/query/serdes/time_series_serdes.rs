use dyn_clone::DynClone;

use crate::common::{
    data::partial_time_series::PartialTimeSeries,
    query::{
        query_node::QueryNode,
        query_node_config::{Builder, QueryNodeConfig},
        query_result::QueryResult,
    },
    stats::span::Span,
};

use super::serde_callback::SerdesCallback;

#[async_trait::async_trait]
pub(crate) trait TimeSeriesSerdes<B, C>
where
    Self: DynClone,
    B: Builder<B, C>,
    C: QueryNodeConfig<B, C>,
{
    async fn serialize(&self, result: Box<dyn QueryResult>, span: Box<dyn Span>);

    async fn serialize_with_partial(
        &self,
        series: Box<dyn PartialTimeSeries>,
        callback: Box<dyn SerdesCallback>,
        span: Box<dyn Span>,
    );

    async fn serialize_complete(&self, span: Box<dyn Span>);

    async fn deserialize(&self, node: Box<dyn QueryNode<B, C>>, span: Box<dyn Span>);
}

impl<B, C> Clone for Box<dyn TimeSeriesSerdes<B, C>>
where
    B: Builder<B, C> + Clone,
    C: QueryNodeConfig<B, C> + Clone,
{
    fn clone(&self) -> Self {
        todo!()
    }
}
