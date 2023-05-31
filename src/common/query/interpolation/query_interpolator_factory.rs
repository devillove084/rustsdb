use crate::common::core::tsdb_plugin::TSDBPlugin;

#[async_trait::async_trait]
pub(crate) trait QueryInterpolatorFactory: TSDBPlugin {}
