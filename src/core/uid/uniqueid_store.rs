use bytes::Bytes;
use dyn_clone::{clone_trait_object, DynClone};

use super::uniqueid_type::UniqueIDType;
use crate::common::{data::time_series_datum_id::TimeSeriesDatumID, stats::span::Span};

#[async_trait::async_trait]
pub(crate) trait UniqueIDStore: DynClone {
    async fn get_id(&self, utype: UniqueIDType, name: String, span: Box<dyn Span>) -> Bytes;

    async fn get_ids(
        &self,
        utype: UniqueIDType,
        names: Vec<String>,
        span: Box<dyn Span>,
    ) -> Vec<Bytes>;

    // TODO: lack idorerror
    async fn get_or_create_id(
        &self,
        utype: UniqueIDType,
        name: String,
        id: Box<dyn TimeSeriesDatumID>,
        span: Box<dyn Span>,
    );

    async fn get_or_create_ids(
        &self,
        utype: UniqueIDType,
        names: Vec<String>,
        id: Box<dyn TimeSeriesDatumID>,
        span: Box<dyn Span>,
    );

    async fn get_name(&self, utype: UniqueIDType, id: Bytes, span: Box<dyn Span>) -> String;

    async fn get_names(
        &self,
        utype: UniqueIDType,
        ids: Vec<Bytes>,
        span: Box<dyn Span>,
    ) -> Vec<String>;

    async fn suggest(&self, utype: UniqueIDType, query: String, max: i32) -> Vec<String>;

    fn character_set(&self, utype: UniqueIDType) -> String;
}

clone_trait_object!(UniqueIDStore);
