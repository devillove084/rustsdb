use bytes::Bytes;

#[derive(Clone)]
pub(crate) struct ConfigurationOverride {
    secret: bool,
    source: String,
    value: Bytes,
    last_change: u64,
}
