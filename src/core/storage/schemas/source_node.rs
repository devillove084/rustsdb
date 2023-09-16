use super::schema::Schema;
use crate::common::data::timestamp::TimeStamp;

pub(crate) trait SourceNode {
    fn get_seq_end(&self) -> Box<dyn TimeStamp>;

    fn get_schema(&self) -> Schema;
}
