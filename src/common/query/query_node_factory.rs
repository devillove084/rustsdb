use dyn_clone::{clone_trait_object, DynClone};

use super::{
    plan::query_plan::QueryPlanner,
    query_node::QueryNode,
    query_node_config::{Builder, QueryNodeConfig},
    query_pipeline_context::QueryPipelineContext,
    util::{JsonNode, ObjectMapper},
};
use crate::common::core::{tsdb::TSDB, tsdb_plugin::TSDBPlugin};

#[async_trait::async_trait]
pub(crate) trait QueryNodeFactory
where
    Self: TSDBPlugin + DynClone,
{
    fn id(&self) -> String;

    async fn parse_config(
        &self,
        mapper: ObjectMapper,
        tsdb: Box<dyn TSDB>,
        node: JsonNode,
    ) -> Box<dyn QueryNodeConfig>;

    async fn setup_graph(
        &self,
        context: Box<dyn QueryPipelineContext>,
        config: dyn QueryNodeConfig,
        planner: Box<dyn QueryPlanner>,
    );

    async fn new_node(&self, context: Box<dyn QueryPipelineContext>) -> Box<dyn QueryNode>;

    async fn new_node_with_config(
        &self,
        context: Box<dyn QueryPipelineContext>,
        config: dyn QueryNodeConfig,
    ) -> Box<dyn QueryNode>;
}

impl Clone for Box<dyn QueryNodeFactory> {
    fn clone(&self) -> Self {
        todo!()
    }
}
