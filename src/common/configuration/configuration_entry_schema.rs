use super::configuration_value_validator::ConfigurationValueValidator;

pub(crate) struct ConfigurationEntrySchema {
    key: String,
    //typ: T,
    source: String,
    description: String,
    validator: Box<dyn ConfigurationValueValidator>,
    default_value: String,
    dynamic: bool,
    nullable: bool,
    secret: bool,
    help_level: String,
    meta: String,
}
