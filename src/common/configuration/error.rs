pub(crate) struct ConfigurationError {}

pub(crate) type ConfigurationResult<T> = std::result::Result<T, ConfigurationError>;
