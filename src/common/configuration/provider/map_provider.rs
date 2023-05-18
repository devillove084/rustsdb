use std::{collections::HashMap, todo};

use crate::common::configuration::configuration_override::ConfigurationOverride;

use super::{provider_factory::ProviderFactory, Provider};

pub struct MapProvider {}

#[async_trait::async_trait]
impl<T> Provider<T> for MapProvider {
    async fn get_setting(&self, key: String) -> ConfigurationOverride<T> {
        todo!()
    }

    async fn source(&self) -> String {
        todo!()
    }

    async fn reload(&self) {
        todo!()
    }

    async fn last_reload(&self) -> u64 {
        todo!()
    }

    async fn factory(&self) -> Box<dyn ProviderFactory<T>> {
        todo!()
    }

    async fn populate_raw_map(&self, map: HashMap<String, String>) {
        todo!()
    }
}
