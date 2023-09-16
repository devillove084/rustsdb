use bytes::Bytes;
use dyn_clone::{clone_trait_object, DynClone};

use super::uniqueid_type::UniqueIDType;
use crate::common::{data::time_series_datum_id::TimeSeriesDatumID, stats::span::Span};

pub(crate) enum UniqueIDCacheMode {
    WRITEONLY(String),
    READONLY(String),
    READWRITE(String),
}

impl Default for UniqueIDCacheMode {
    fn default() -> Self {
        todo!()
    }
}

impl UniqueIDCacheMode {
    pub fn get_name(&self) -> String {
        todo!()
    }

    pub fn from_string(&self, name: String) -> Self {
        todo!()
    }

    pub fn set_mode(&mut self, name: String) {
        todo!()
    }
}

#[async_trait::async_trait]
pub(crate) trait UniqueID: DynClone {
    fn get_type(&self) -> UniqueIDType;

    async fn drop_caches(&self, span: Box<dyn Span>);

    fn get_name(&self, uid: u64, span: Box<dyn Span>) -> String;

    fn get_names(&self, uid: u64, span: Box<dyn Span>) -> Vec<String>;

    fn get_id(&self, name: String, span: Box<dyn Span>) -> Bytes;

    fn get_ids(&self, names: Vec<String>, span: Box<dyn Span>) -> Vec<Bytes>;

    // TODO: lack return idorerror
    fn get_or_create_id(&self, name: String, id: Box<dyn TimeSeriesDatumID>, span: Box<dyn Span>);

    // TODO: lack return vec of idorerror
    fn get_or_create_ids(
        &self,
        name: Vec<String>,
        id: Box<dyn TimeSeriesDatumID>,
        span: Box<dyn Span>,
    );

    fn suggest(&self, search: String, max_results: i32, span: Box<dyn Span>) -> Vec<String>;

    fn rename(&self, oldname: String, newname: String, span: Box<dyn Span>);

    fn add_to_cache(&self, name: String, id: Bytes);

    // TODO: need a result
    async fn delete(&self, name: String, span: Box<dyn Span>);

    // TODO: lack convert funcs!!!
}

clone_trait_object!(UniqueID);
