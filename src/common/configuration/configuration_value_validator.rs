use super::{configuration_entry_schema::ConfigurationEntrySchema, error::ConfigurationResult};

#[async_trait::async_trait]
pub(crate) trait ConfigurationValueValidator<T> {
    async fn validate(
        &self,
        schema: ConfigurationEntrySchema<T>,
    ) -> ConfigurationResult<Validation>;
}

pub(crate) struct Validation {
    valid: bool,
    message: String,
}
