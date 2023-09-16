use std::collections::HashMap;

use super::{query_node_config_options::QueryNodeConfigOptions, query_result_id::QueryResultID};
use crate::common::configuration::configuration::Configuration;

#[async_trait::async_trait]
pub(crate) trait Builder {
    fn set_id(&self, id: String) -> Box<dyn Builder>;

    fn set_type(&self, typ: String) -> Box<dyn Builder>;

    fn set_sources(&self, sources: Vec<String>) -> Box<dyn Builder>;

    fn add_source(&self, source: String) -> Box<dyn Builder>;

    fn set_overrides(&self, overrides: HashMap<String, String>) -> Box<dyn Builder>;

    fn add_overrides(&self, key: String, value: String) -> Box<dyn Builder>;

    fn set_result_ids(&self, result_ids: Vec<Box<dyn QueryResultID>>) -> Box<dyn Builder>;

    fn add_result_id(&self, result_id: Box<dyn QueryResultID>) -> Box<dyn Builder>;

    async fn build(&self) -> Box<dyn Builder>;

    fn return_self(&self) -> Box<dyn Builder>;
}

#[async_trait::async_trait]
pub(crate) trait GetQueryNodeOption<T> {
    async fn node_options(option: QueryNodeConfigOptions) -> T;
}

pub(crate) trait QueryNodeConfig {
    fn get_id(&self) -> String;

    fn get_type(&self) -> String;

    fn get_sources(&self) -> Vec<String>;

    fn is_push_down(&self) -> bool;

    fn is_joins(&self) -> bool;

    fn is_node_flag(&self, option: QueryNodeConfigOptions) -> bool;

    fn is_read_cache(&self) -> bool;

    fn get_overrides(&self) -> HashMap<String, String>;

    fn get_string(&self, config: Configuration, key: String) -> String;

    fn get_int(&self, config: Configuration, key: String) -> i32;

    fn get_long(&self, config: Configuration, key: String) -> u64;

    fn get_bool(&self, config: Configuration, key: String) -> bool;

    fn get_double(&self, config: Configuration, key: String) -> u32;

    fn has_key(&self, key: String) -> String;

    fn to_builder(&self) -> Box<dyn Builder>;

    fn result_ids(&self) -> Vec<Box<dyn QueryResultID>>;

    fn is_marked_cacheable(&self) -> bool;

    fn mark_cacheable(&self);
}

// impl Hash for dyn QueryNodeConfig {
//     fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
//         todo!()
//     }
// }

// impl PartialEq for dyn QueryNodeConfig {
//     fn eq(&self, _other: &Self) -> bool {
//         todo!()
//     }
// }

// impl Eq for dyn QueryNodeConfig {}
