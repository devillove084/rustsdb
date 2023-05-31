use dyn_clone::{clone_trait_object, DynClone};

use crate::common::core::tsdb_plugin::TSDBPlugin;

pub(crate) trait TSDBThreadPoolExecutor: DynClone + TSDBPlugin {
    // TODO:
}

clone_trait_object!(TSDBThreadPoolExecutor);
