use super::timestamp::TimeStamp;

pub(crate) trait TimeSpecification {
    fn start(&self) -> Box<dyn TimeStamp>;

    fn end(&self) -> Box<dyn TimeStamp>;

    fn interval(&self) -> u64;

    fn string_interval(&self) -> String;
}
