use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To supply patient information

See docs at <https://www.stedi.com/edi/x12/segment/PAT>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "PAT")]
pub struct Patient {
    /**PAT-01 (1069)
Code indicating the relationship between two individuals or entities*/
    pub individual_relationship_code: Option<Fixed<2>>,
    /**PAT-02 (1384)
Code identifying the location where patient is receiving medical treatment*/
    pub patient_location_code: Option<Fixed<1>>,
    /**PAT-03 (584)
Code showing the general employment status of an employee/claimant*/
    pub employment_status_code: Option<Fixed<2>>,
    /**PAT-04 (1220)
Code indicating the student status of the patient if 19 years of age or older, not handicapped and not the insured*/
    pub student_status_code: Option<Fixed<1>>,
    /**PAT-05 (1250)
Code indicating the date format, time format, or date and time format*/
    pub date_time_period_format_qualifier: Option<String>,
    /**PAT-06 (1251)
Expression of a date, a time, or range of dates, times or dates and times*/
    pub date_time_period: Option<String>,
    /**PAT-07 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub unit_or_basis_for_measurement_code: Option<Fixed<2>>,
    /**PAT-08 (81)
Numeric value of weight*/
    pub weight: Option<String>,
    /**PAT-09 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Option<Fixed<1>>,
}
impl Patient {}