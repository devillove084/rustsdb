use crate::common::data::partial_time_series::PartialTimeSeries;

pub(crate) trait SerdesCallback {
    fn on_complete(&self, pts: Box<dyn PartialTimeSeries>);

    fn on_error(&self, pts: Box<dyn PartialTimeSeries>);
}
