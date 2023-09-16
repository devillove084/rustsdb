use std::collections::{HashMap, HashSet};

use petgraph::Graph;

use super::context_node_config::ContextNodeConfig;
use crate::common::{
    data::{
        time_series_data_source::TimeSeriesDataSource,
        time_series_data_source_config::TimeSeriesDataSourceConfig,
    },
    query::{
        plan::query_plan::{QueryPlanner, TimeAdjustments},
        query_node::QueryNode,
        query_node_config::{Builder, QueryNodeConfig},
        query_node_factory::QueryNodeFactory,
        query_pipeline_context::QueryPipelineContext,
        query_result_id::QueryResultID,
    },
    stats::span::Span,
};

pub(crate) struct DefaultQueryPlanner {
    context: Box<dyn QueryPipelineContext>,
    context_sink: Box<dyn QueryNode>,
    context_sink_config: ContextNodeConfig,
    sink_filter: HashMap<String, String>,
    roots: Vec<Box<dyn QueryNodeConfig>>,
    graph: Graph<Box<dyn QueryNode>, Box<dyn QueryNode>>,
    data_sources: Vec<Box<dyn TimeSeriesDataSource>>,
    source_node: HashSet<Box<dyn QueryNodeConfig>>,
    config_graph: Graph<Box<dyn QueryNode>, Box<dyn QueryNode>>,
    nodes_map: HashMap<String, Box<dyn QueryNode>>,
    factory_cache: HashMap<String, Box<dyn QueryNodeFactory>>,
    context_node: Box<dyn QueryNodeConfig>,
    serialization_sources: Vec<Box<dyn QueryResultID>>,
    modified_during_setup: bool,
    satisfied_filter: HashSet<String>,
}

unsafe impl Send for DefaultQueryPlanner {}

unsafe impl Sync for DefaultQueryPlanner {}

#[async_trait::async_trait]
impl QueryPlanner for DefaultQueryPlanner {
    async fn plan(&self, span: Box<dyn Span + Send>) {
        todo!()
    }

    async fn replace(
        &self,
        old_config: Box<dyn QueryNodeConfig + Send>,
        new_config: Box<dyn QueryNodeConfig + Send>,
    ) {
        todo!()
    }

    async fn add_edge(
        &self,
        from: Box<dyn QueryNodeConfig + Send>,
        to: Box<dyn QueryNodeConfig + Send>,
    ) -> bool {
        todo!()
    }

    async fn remove_edge(
        &self,
        from: Box<dyn QueryNodeConfig + Send>,
        to: Box<dyn QueryNodeConfig + Send>,
    ) -> bool {
        todo!()
    }

    async fn remove_node(&self, config: Box<dyn QueryNodeConfig + Send>) -> bool {
        todo!()
    }

    fn graph(&self) -> Graph<Box<dyn QueryNode>, Box<dyn QueryNode>> {
        todo!()
    }

    fn config_graph(&self) -> Graph<Box<dyn QueryNodeConfig>, Box<dyn QueryNodeConfig>> {
        todo!()
    }

    fn context(&self) -> Box<dyn QueryPipelineContext> {
        todo!()
    }

    async fn terminal_source_node(
        &self,
        config: Box<dyn QueryNodeConfig + Send>,
    ) -> Vec<Box<dyn QueryNodeConfig + Send>> {
        todo!()
    }

    async fn base_setup_graph(
        &self,
        context: Box<dyn QueryPipelineContext + Send>,
        config: Box<dyn TimeSeriesDataSourceConfig + Send>,
    ) {
        todo!()
    }

    fn node_for_id(&self, id: String) -> Box<dyn QueryNode> {
        todo!()
    }

    fn get_factory(&self, node: Box<dyn QueryNodeConfig>) -> Box<dyn QueryNodeFactory> {
        todo!()
    }

    fn get_metric_for_data_source(
        &self,
        node: Box<dyn QueryNodeConfig>,
        data_source_id: String,
    ) -> String {
        todo!()
    }

    fn get_adjustments(&self, config: Box<dyn TimeSeriesDataSourceConfig>) -> TimeAdjustments {
        todo!()
    }
}
