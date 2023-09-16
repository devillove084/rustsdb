use std::collections::HashSet;

use super::{
    configuration::Configuration, configuration_callback::ConfigurationCallback,
    configuration_entry_schema::ConfigurationEntrySchema,
    configuration_override::ConfigurationOverride,
};

pub(crate) struct ConfigurationEntry {
    config: Configuration,
    schema: ConfigurationEntrySchema,
    settings: Vec<ConfigurationOverride>,
    callbacks: HashSet<Box<dyn ConfigurationCallback>>,
}

impl ConfigurationEntry {
    pub fn schema(&self) -> &ConfigurationEntrySchema {
        &self.schema
    }
}
