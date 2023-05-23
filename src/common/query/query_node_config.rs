use std::collections::HashMap;

use crate::common::configuration::Configuration;

use super::{query_node_config_options::QueryNodeConfigOptions, query_result_id::QueryResultID};

#[async_trait::async_trait]
pub(crate) trait Builder<B, C>
where
    B: Builder<B, C>,
    C: QueryNodeConfig<B, C>,
{
    fn set_id(&self, id: String) -> B;

    fn set_type(&self, typ: String) -> B;

    fn set_sources(&self, sources: Vec<String>) -> B;

    fn add_source(&self, source: String) -> B;

    fn set_overrides(&self, overrides: HashMap<String, String>) -> B;

    fn add_overrides(&self, key: String, value: String) -> B;

    fn set_result_ids(&self, result_ids: Vec<Box<dyn QueryResultID>>) -> B;

    fn add_result_id(&self, result_id: Box<dyn QueryResultID>) -> B;

    async fn build(&self) -> B;

    // TODO: May not be a good enough design
    fn return_self(&self) -> B;
}

#[async_trait::async_trait]
pub(crate) trait QueryNodeConfig<B, C>
where
    B: Builder<B, C>,
    C: QueryNodeConfig<B, C>,
{
    fn get_id(&self) -> String;

    fn get_type(&self) -> String;

    fn get_sources(&self) -> Vec<String>;

    fn is_push_down(&self) -> bool;

    fn is_joins(&self) -> bool;

    // async fn node_option<U>(&self, option: QueryNodeConfigOptions) -> U;

    fn is_node_flag(&self, option: QueryNodeConfigOptions) -> bool;

    fn is_read_cache(&self) -> bool;

    async fn get_overrides(&self) -> HashMap<String, String>;

    fn get_string(&self, config: Configuration, key: String) -> String;

    fn get_int(&self, config: Configuration, key: String) -> i32;

    fn get_long(&self, config: Configuration, key: String) -> u64;

    fn get_bool(&self, config: Configuration, key: String) -> bool;

    fn get_double(&self, config: Configuration, key: String) -> u32;

    fn has_key(&self, key: String) -> String;

    async fn to_builder(&self) -> B;

    fn result_ids(&self) -> Vec<Box<dyn QueryResultID>>;

    fn is_marked_cacheable(&self) -> bool;

    async fn mark_cacheable(&self);
}

// TODO: reimplentment them
// impl<B: Builder<B, T>, T> Hash for dyn QueryNodeConfig<B, T> {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         todo!()
//     }
// }

// impl PartialEq for dyn QueryNodeConfig {
//     fn eq(&self, other: &Self) -> bool {
//         todo!()
//     }
// }

// impl Eq for dyn QueryNodeConfig {}
