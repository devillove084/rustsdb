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
pub(crate) trait QueryPlanner<B, C>
where
    B: Builder<B, C>,
    C: QueryNodeConfig<B, C>,
{
    async fn plan(&self, span: Box<dyn Span>);

    async fn replace(
        &self,
        old_config: Box<dyn QueryNodeConfig<B, C>>,
        new_config: Box<dyn QueryNodeConfig<B, C>>,
    );

    async fn add_edge(
        &self,
        from: Box<dyn QueryNodeConfig<B, C>>,
        to: Box<dyn QueryNodeConfig<B, C>>,
    ) -> bool;

    async fn remove_edge(
        &self,
        from: Box<dyn QueryNodeConfig<B, C>>,
        to: Box<dyn QueryNodeConfig<B, C>>,
    ) -> bool;

    async fn remove_node(&self, config: Box<dyn QueryNodeConfig<B, C>>) -> bool;

    async fn graph(&self) -> Graph<Box<dyn QueryNode<B, C>>, Box<dyn QueryNode<B, C>>>;

    async fn config_graph(
        &self,
    ) -> Graph<Box<dyn QueryNodeConfig<B, C>>, Box<dyn QueryNodeConfig<B, C>>>;

    async fn context(&self) -> Box<dyn QueryPipelineContext<B, C>>;

    async fn node_for_id(&self, id: String) -> Box<dyn QueryNode<B, C>>;

    async fn get_factory(
        &self,
        node: Box<dyn QueryNodeConfig<B, C>>,
    ) -> Box<dyn QueryNodeFactory<B, C>>;

    async fn terminal_source_node(
        &self,
        config: Box<dyn QueryNodeConfig<B, C>>,
    ) -> Vec<Box<dyn QueryNodeConfig<B, C>>>;

    async fn get_metric_for_data_source(
        &self,
        node: Box<dyn QueryNodeConfig<B, C>>,
        data_source_id: String,
    ) -> String;

    async fn get_adjustments(
        &self,
        config: Box<dyn TimeSeriesDataSourceConfig<B, C>>,
    ) -> TimeAdjustments;

    async fn base_setup_graph(
        &self,
        context: Box<dyn QueryPipelineContext<B, C>>,
        config: Box<dyn TimeSeriesDataSourceConfig<B, C>>,
    );
}
