use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To specify any or all of a date, a time, or a time period

See docs at <https://www.stedi.com/edi/x12-005010/segment/DTP>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "DTP")]
pub struct DateOrTimeOrPeriod {
    /**DTP-01 (374)
Code specifying type of date or time, or both date and time*/
    pub date_time_qualifier: Fixed<3>,
    /**DTP-02 (1250)
Code indicating the date format, time format, or date and time format*/
    pub date_time_period_format_qualifier: String,
    /**DTP-03 (1251)
Expression of a date, a time, or range of dates, times or dates and times*/
    pub date_time_period: String,
}
impl DateOrTimeOrPeriod {}