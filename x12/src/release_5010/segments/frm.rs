use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To specify information in response to a codified questionnaire document

See docs at <https://www.stedi.com/edi/x12-005010/segment/FRM>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "FRM")]
pub struct SupportingDocumentation {
    /**FRM-01 (350)
Alphanumeric characters assigned for differentiation within a transaction set*/
    pub assigned_identification: String,
    /**FRM-02 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Option<Fixed<1>>,
    /**FRM-03 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub reference_identification: Option<String>,
    /**FRM-04 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub date: Option<Fixed<8>>,
    /**FRM-05 (332)
Percent given in decimal format (e.g., 0.0 through 100.0 represents 0% through 100%)*/
    pub percent: Option<String>,
}
impl SupportingDocumentation {}