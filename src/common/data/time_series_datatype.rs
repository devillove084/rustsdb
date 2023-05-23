/// Describes a type of data for use in storing and querying with OpenTSDB.
pub(crate) trait TimeSeriesDataType {
    /// The non-null type of data this value represents.
    fn get_type(&self) -> Box<dyn TimeSeriesDataType>;
}
