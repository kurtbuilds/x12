use serde::{Serialize, Deserialize};
/**To specify the status of individual teeth

See docs at <https://www.stedi.com/edi/x12/segment/DN2>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "DN2")]
pub struct ToothSummary {
    /**DN2-01 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub reference_identification: String,
    /**DN2-02 (1368)
Code specifying the status of the tooth*/
    pub tooth_status_code: Option<String>,
    /**DN2-03 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**DN2-04 (1250)
Code indicating the date format, time format, or date and time format*/
    pub date_time_period_format_qualifier: Option<String>,
    /**DN2-05 (1251)
Expression of a date, a time, or range of dates, times or dates and times*/
    pub date_time_period: Option<String>,
    /**DN2-06 (1270)
Code identifying a specific industry code list*/
    pub code_list_qualifier_code: String,
}
impl ToothSummary {}