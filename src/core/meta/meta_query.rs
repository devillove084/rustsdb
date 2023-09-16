use crate::common::query::filter::query_filter::QueryFilter;

pub(crate) trait MetaQuery {
    fn get_namespace(&self) -> String;

    fn get_filter(&self) -> Box<dyn QueryFilter>;

    fn get_id(&self) -> String;
}

pub(crate) struct MetaQueryBuilder {
    namespace: String,
    filter: Box<dyn QueryFilter>,
    id: String,
}

impl MetaQueryBuilder {
    pub fn set_namespace(&mut self, namespace: String) {
        self.namespace = namespace
    }

    pub fn set_filter(&mut self, filter: Box<dyn QueryFilter>) {
        self.filter = filter
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id
    }

    pub fn build(&self) {
        todo!()
    }
}
