use std::collections::HashSet;

use super::{
    configuration_callback::ConfigurationCallback,
    configuration_entry_schema::ConfigurationEntrySchema,
    configuration_override::ConfigurationOverride, Configuration,
};

pub(crate) struct ConfigurationEntry {
    config: Configuration,
    schema: ConfigurationEntrySchema,
    settings: Vec<ConfigurationOverride>,
    callbacks: HashSet<Box<dyn ConfigurationCallback>>,
}
