use dyn_clone::{clone_trait_object, DynClone};

use crate::common::storage::time_series_data_consumer::TimeSeriesDataConsumer;

pub(crate) trait TSDBDataStore: TimeSeriesDataConsumer + DynClone {
    fn id(&self) -> String;
}

clone_trait_object!(TSDBDataStore);
