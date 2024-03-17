use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
use super::super::codes::CodeListQualifierCode;
use super::super::codes::DateTimePeriodFormatQualifier;
/**To send health care codes and their associated dates, amounts and quantities

See docs at <https://www.stedi.com/edi/x12/element/C022>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C022")]
pub struct HealthCareCode {
    /**C022-01 (1270)
Code identifying a specific industry code list*/
    pub health_care_code: CodeListQualifierCode,
    /**C022-02 (1271)
Code indicating a code from a specific industry code list*/
    pub c022_02: String,
    /**C022-03 (1250)
Code indicating the date format, time format, or date and time format*/
    pub c022_03: Option<DateTimePeriodFormatQualifier>,
    /**C022-04 (1251)
Expression of a date, a time, or range of dates, times or dates and times*/
    pub c022_04: Option<String>,
    /**C022-05 (782)
Monetary amount*/
    pub c022_05: Option<String>,
    /**C022-06 (380)
Numeric value of quantity*/
    pub c022_06: Option<String>,
    /**C022-07 (799)
Revision level of a particular format, program, technique or algorithm*/
    pub c022_07: Option<String>,
    /**C022-08 (1271)
Code indicating a code from a specific industry code list*/
    pub c022_08: Option<String>,
    /**C022-09 (1073)
Code indicating a Yes or No condition or response*/
    pub c022_09: Option<Fixed<1>>,
}
impl HealthCareCode {}