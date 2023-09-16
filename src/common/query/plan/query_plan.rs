use std::path::Display;

use petgraph::Graph;

use crate::common::{
    data::time_series_data_source_config::TimeSeriesDataSourceConfig,
    query::{
        query_node::QueryNode,
        query_node_config::{Builder, QueryNodeConfig},
        query_node_factory::QueryNodeFactory,
        query_pipeline_context::QueryPipelineContext,
    },
    stats::span::Span,
};

pub(crate) struct TimeAdjustments {
    downsample_interval: String,
    window_interval: String,
    previous_intervals: i32,
}

// impl TimeAdjustments {
//     pub fn to_string(&self) -> String {
//         "ds=".to_owned()
//             + &self.downsample_interval
//             + ", wi="
//             + &self.window_interval
//             + ", pi="
//             + &self.previous_intervals.to_string()
//     }
// }

#[async_trait::async_trait]
pub(crate) trait QueryPlanner {
    async fn plan(&self, span: Box<dyn Span + Send>);

    async fn replace(
        &self,
        old_config: Box<dyn QueryNodeConfig + Send>,
        new_config: Box<dyn QueryNodeConfig + Send>,
    );

    async fn add_edge(
        &self,
        from: Box<dyn QueryNodeConfig + Send>,
        to: Box<dyn QueryNodeConfig + Send>,
    ) -> bool;

    async fn remove_edge(
        &self,
        from: Box<dyn QueryNodeConfig + Send>,
        to: Box<dyn QueryNodeConfig + Send>,
    ) -> bool;

    async fn remove_node(&self, config: Box<dyn QueryNodeConfig + Send>) -> bool;

    fn graph(&self) -> Graph<Box<dyn QueryNode>, Box<dyn QueryNode>>;

    fn config_graph(&self) -> Graph<Box<dyn QueryNodeConfig>, Box<dyn QueryNodeConfig>>;

    fn context(&self) -> Box<dyn QueryPipelineContext>;

    fn node_for_id(&self, id: String) -> Box<dyn QueryNode>;

    fn get_factory(&self, node: Box<dyn QueryNodeConfig>) -> Box<dyn QueryNodeFactory>;

    async fn terminal_source_node(
        &self,
        config: Box<dyn QueryNodeConfig + Send>,
    ) -> Vec<Box<dyn QueryNodeConfig + Send>>;

    fn get_metric_for_data_source(
        &self,
        node: Box<dyn QueryNodeConfig>,
        data_source_id: String,
    ) -> String;

    fn get_adjustments(&self, config: Box<dyn TimeSeriesDataSourceConfig>) -> TimeAdjustments;

    async fn base_setup_graph(
        &self,
        context: Box<dyn QueryPipelineContext + Send>,
        config: Box<dyn TimeSeriesDataSourceConfig + Send>,
    );
}
