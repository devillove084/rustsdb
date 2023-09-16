use std::collections::HashMap;

use crate::common::{
    configuration::configuration::Configuration,
    query::{
        query_node_config::{Builder, QueryNodeConfig},
        query_node_config_options::QueryNodeConfigOptions,
        query_result_id::QueryResultID,
    },
};

pub(crate) struct ContextNodeConfig {}

impl QueryNodeConfig for ContextNodeConfig {
    fn get_id(&self) -> String {
        todo!()
    }

    fn get_type(&self) -> String {
        todo!()
    }

    fn get_sources(&self) -> Vec<String> {
        todo!()
    }

    fn is_push_down(&self) -> bool {
        todo!()
    }

    fn is_joins(&self) -> bool {
        todo!()
    }

    fn is_node_flag(&self, option: QueryNodeConfigOptions) -> bool {
        todo!()
    }

    fn is_read_cache(&self) -> bool {
        todo!()
    }

    fn get_overrides(&self) -> HashMap<String, String> {
        todo!()
    }

    fn get_string(&self, config: Configuration, key: String) -> String {
        todo!()
    }

    fn get_int(&self, config: Configuration, key: String) -> i32 {
        todo!()
    }

    fn get_long(&self, config: Configuration, key: String) -> u64 {
        todo!()
    }

    fn get_bool(&self, config: Configuration, key: String) -> bool {
        todo!()
    }

    fn get_double(&self, config: Configuration, key: String) -> u32 {
        todo!()
    }

    fn has_key(&self, key: String) -> String {
        todo!()
    }

    fn to_builder(&self) -> Box<dyn Builder> {
        todo!()
    }

    fn result_ids(&self) -> Vec<Box<dyn QueryResultID>> {
        todo!()
    }

    fn is_marked_cacheable(&self) -> bool {
        todo!()
    }

    fn mark_cacheable(&self) {
        todo!()
    }
}
