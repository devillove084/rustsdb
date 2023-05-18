pub(crate) struct ConfigurationOverride<T> {
    secret: bool,
    source: String,
    value: T,
    last_change: u64,
}
