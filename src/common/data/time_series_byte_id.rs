pub(crate) trait TimeSeriesID {
    // True if the fields are encoded using a format specified by a storage engine.
    fn encode(&self) -> bool;

    // A signed 64 bit hash code for collision reduction.
    // return a hash as u64
    fn hashcode(&self) -> u64;

    // The type series dealt with. Either a `TimeSeriesByteID` or `TimeSeriesStringID`.
    // fn get_type<T: TimeSeriesID>(&self, _: T) -> &str;
}
