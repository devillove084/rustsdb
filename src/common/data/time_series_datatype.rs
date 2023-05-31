use dyn_clone::{clone_trait_object, DynClone};

/// Describes a type of data for use in storing and querying with OpenTSDB.
pub(crate) trait TimeSeriesDataType: DynClone {
    /// The non-null type of data this value represents.
    fn get_type(&self) -> Box<dyn TimeSeriesDataType>;
}

clone_trait_object!(TimeSeriesDataType);
