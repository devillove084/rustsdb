use std::collections::HashSet;

use super::{
    configuration_callback::ConfigurationCallback,
    configuration_entry_schema::ConfigurationEntrySchema,
    configuration_override::ConfigurationOverride, Configuration,
};

pub(crate) struct ConfigurationEntry<T> {
    config: Configuration<T>,
    schema: ConfigurationEntrySchema<T>,
    settings: Vec<ConfigurationOverride<T>>,
    callbacks: HashSet<Box<dyn ConfigurationCallback<T>>>,
}
