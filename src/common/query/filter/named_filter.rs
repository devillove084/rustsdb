use super::query_filter::QueryFilter;

pub(crate) trait NamedFilter {
    fn get_id(&self) -> String;

    fn get_filter(&self) -> Box<dyn QueryFilter>;
}
