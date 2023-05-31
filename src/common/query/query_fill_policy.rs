use crate::common::data::time_series_datatype::TimeSeriesDataType;

use super::interpolation::query_interpolator_config::QueryInterpolatorConfig;

pub(crate) enum FillWithRealPolicy {
    NONE,
    PREVIOUSONLY,
    NEXTONLY,
    PREFERPREVIOUS,
    PREFERNEXT,
}

#[async_trait::async_trait]
pub(crate) trait QueryFillPolicy {
    async fn fill(&self) -> Box<dyn TimeSeriesDataType>;

    fn real_policy(&self) -> FillWithRealPolicy;

    fn config(&self) -> Box<dyn QueryInterpolatorConfig>;
}
