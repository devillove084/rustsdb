use std::hash::Hash;

use super::serdes::serdes_options::SerdesOptions;

pub(crate) trait QuerySinkConfig {
    fn get_id(&self) -> String;

    fn serdes_options(&self) -> Box<dyn SerdesOptions>;
}

impl Hash for dyn QuerySinkConfig {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        todo!()
    }
}
