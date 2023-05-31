use dyn_clone::{clone_trait_object, DynClone};

use crate::common::core::{tsdb::TSDB, tsdb_plugin::TSDBPlugin};

use super::{
    plan::query_plan::QueryPlanner,
    query_node::QueryNode,
    query_node_config::{Builder, QueryNodeConfig},
    query_pipeline_context::QueryPipelineContext,
    util::{JsonNode, ObjectMapper},
};

#[async_trait::async_trait]
pub(crate) trait QueryNodeFactory<B, C>
where
    Self: TSDBPlugin + DynClone,
    B: Builder<B, C>,
    C: QueryNodeConfig<B, C>,
{
    fn id(&self) -> String;

    async fn parse_config(&self, mapper: ObjectMapper, tsdb: Box<dyn TSDB>, node: JsonNode) -> C;

    async fn setup_graph(
        &self,
        context: Box<dyn QueryPipelineContext<B, C>>,
        config: C,
        planner: Box<dyn QueryPlanner<B, C>>,
    );

    async fn new_node(
        &self,
        context: Box<dyn QueryPipelineContext<B, C>>,
    ) -> Box<dyn QueryNode<B, C>>;

    async fn new_node_with_config(
        &self,
        context: Box<dyn QueryPipelineContext<B, C>>,
        config: C,
    ) -> Box<dyn QueryNode<B, C>>;
}

impl<B, C> Clone for Box<dyn QueryNodeFactory<B, C>>
where
    B: Builder<B, C> + Clone,
    C: QueryNodeConfig<B, C> + Clone,
{
    fn clone(&self) -> Self {
        todo!()
    }
}
