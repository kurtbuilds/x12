use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To uniquely identify a transaction to an application

See docs at <https://www.stedi.com/edi/hipaa/segment/TRN>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "TRN")]
pub struct Trace {
    /**TRN-01 (481)
Code identifying which transaction is being referenced*/
    pub trace_type_code: String,
    /**TRN-02 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub reference_identification: String,
    /**TRN-03 (509)
A unique identifier designating the company initiating the funds transfer instructions, business transaction or assigning tracking reference identification.*/
    pub originating_company: Option<Fixed<10>>,
    /**TRN-04 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub trn04: Option<String>,
}
impl Trace {}