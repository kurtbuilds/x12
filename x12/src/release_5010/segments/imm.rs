use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To provide the receiving school district or postsecondary institution with a notice of the immunization status of the student

See docs at <https://www.stedi.com/edi/x12-005010/segment/IMM>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "IMM")]
pub struct ImmunizationStatus {
    /**IMM-01 (1271)
Code indicating a code from a specific industry code list*/
    pub industry_code: String,
    /**IMM-02 (1250)
Code indicating the date format, time format, or date and time format*/
    pub date_time_period_format_qualifier: Option<String>,
    /**IMM-03 (1251)
Expression of a date, a time, or range of dates, times or dates and times*/
    pub date_time_period: Option<String>,
    /**IMM-04 (1254)
Code indicating the status of an immunization conducted on a person*/
    pub immunization_status_code: Option<String>,
    /**IMM-05 (755)
Code indicating the title or contents of a document, report or supporting item*/
    pub report_type_code: Option<Fixed<2>>,
    /**IMM-06 (1270)
Code identifying a specific industry code list*/
    pub code_list_qualifier_code: String,
}
impl ImmunizationStatus {}