use super::Comparable;

/// Provides common methods for interacting with timestamps.
/// Implementations can provide as high or low resolution as desired.
/// The `units` describe what resolution this timestamps is encoded at.
/// Possibel values from `ChronoUnit#SECONDS` to `ChronoUnit#NANOS`
pub(crate) trait TimeStamp {
    fn nanos(&self) -> u64;

    fn ms_epoch(&self) -> u64;

    fn epoch(&self) -> u64;

    fn update_ms_epoch(&self, ts: u64);

    fn update_epoch(&self, ts: u64);

    fn update_timestamp(&self, ts: dyn TimeStamp);

    fn update_epoch_with_nano(&self, epoch: u64, nano: u64);

    // fn getTimeZone(&self) -> ZoneID;

    fn set_max(&self);

    fn units(&self) -> u64;

    fn add(&self, amount: u64);

    fn subtract(&self, amound: u64);

    fn snap_to_previous_interval(&self, interval: u64, unit: u64);

    fn snap_to_previous_interval_with_day_of_week(&self, interval: u64, unit: u64, dayofweek: u64);
}
