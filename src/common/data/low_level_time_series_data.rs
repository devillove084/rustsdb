use bytes::Bytes;

use super::timestamp::TimeStamp;

pub(crate) enum StringFormat {
    ASCIISTRING,
    UTF8STRING,
    ENCODED,
}

pub(crate) trait LowLevelTimeSeriesData {
    fn advance(&self) -> bool;

    fn has_parsing_error(&self) -> bool;

    fn parsing_error(&self) -> String;

    fn timestamp(&self) -> Box<dyn TimeStamp>;

    fn tags_buffer(&self) -> Vec<Bytes>;

    fn tag_buffer_start(&self) -> i32;

    fn tag_buffer_length(&self) -> i32;

    fn tags_format(&self) -> StringFormat;

    fn tag_delimiter(&self) -> Bytes;

    fn tag_set_count(&self) -> i32;

    fn advance_tag_pair(&self) -> bool;

    fn tag_key_start(&self) -> i32;

    fn tag_key_length(&self) -> i32;

    fn tag_value_start(&self) -> i32;

    fn tag_value_length(&self) -> i32;

    fn common_tags(&self) -> bool;

    fn common_timestamp(&self) -> bool;
}

pub(crate) trait HashedLowLevelTimeSeriesData: LowLevelTimeSeriesData {
    fn time_series_hash(&self) -> u64;

    fn tags_set_hash(&self) -> u64;

    fn tag_pair_hash(&self) -> u64;

    fn tag_key_hash(&self) -> u64;

    fn tag_value_hash(&self) -> u64;
}

pub(crate) trait NamespacedLowLevelTimeSeriesData: LowLevelTimeSeriesData {
    fn namespace_buffer(&self) -> Vec<Bytes>;

    fn namespace_start(&self) -> i32;

    fn namespace_length(&self) -> i32;

    fn namespace_format(&self) -> StringFormat;
}

pub(crate) trait HashedNamespacedLowLevelTimeSeriesData:
    NamespacedLowLevelTimeSeriesData + HashedLowLevelTimeSeriesData
{
    fn namespace_hash(&self) -> u64;
}

pub(crate) trait RetiredLowLevelTimeSeriesData: LowLevelTimeSeriesData {
    fn initial_epoch(&self) -> i32;
}

pub(crate) trait RetiredHashedLowLevelTimeSeriesData: HashedLowLevelTimeSeriesData {
    fn initial_epoch(&self) -> i32;
}

pub(crate) trait NamespacedRetiredHashedLowLevelTimeSeriesData:
    NamespacedLowLevelTimeSeriesData + HashedLowLevelTimeSeriesData
{
    fn initial_epoch(&self) -> i32;
}
