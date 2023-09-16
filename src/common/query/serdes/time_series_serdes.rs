use dyn_clone::{clone_trait_object, DynClone};

use super::serde_callback::SerdesCallback;
use crate::common::{
    data::partial_time_series::PartialTimeSeries,
    query::{query_node::QueryNode, query_result::QueryResult},
    stats::span::Span,
};

#[async_trait::async_trait]
pub(crate) trait TimeSeriesSerdes: DynClone {
    async fn serialize(&self, result: Box<dyn QueryResult>, span: Box<dyn Span>);

    async fn serialize_with_partial(
        &self,
        series: Box<dyn PartialTimeSeries>,
        callback: Box<dyn SerdesCallback>,
        span: Box<dyn Span>,
    );

    async fn serialize_complete(&self, span: Box<dyn Span>);

    async fn deserialize(&self, node: Box<dyn QueryNode>, span: Box<dyn Span>);
}

clone_trait_object!(TimeSeriesSerdes);
