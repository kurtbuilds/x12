use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To supply information related to Pacemaker registry

See docs at <https://www.stedi.com/edi/x12/segment/CR8>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "CR8")]
pub struct PacemakerCertification {
    /**CR8-01 (1403)
Code identifying implant components*/
    pub implant_type_code: Fixed<1>,
    /**CR8-02 (1404)
Code identifying the status of implant components*/
    pub implant_status_code: Fixed<1>,
    /**CR8-03 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub date: Fixed<8>,
    /**CR8-04 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub cr8_04: Fixed<8>,
    /**CR8-05 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub reference_identification: String,
    /**CR8-06 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub cr8_06: String,
    /**CR8-07 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub cr8_07: String,
    /**CR8-08 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Fixed<1>,
    /**CR8-09 (1073)
Code indicating a Yes or No condition or response*/
    pub cr8_09: Fixed<1>,
}
impl PacemakerCertification {}