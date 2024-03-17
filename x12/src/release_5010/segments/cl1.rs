use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To supply information specific to hospital claims

See docs at <https://www.stedi.com/edi/x12/segment/CL1>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "CL1")]
pub struct ClaimCodes {
    /**CL1-01 (1315)
Code indicating the priority of this admission*/
    pub admission_type_code: Option<Fixed<1>>,
    /**CL1-02 (1314)
Code indicating the source of this admission*/
    pub admission_source_code: Option<Fixed<1>>,
    /**CL1-03 (1352)
Code indicating patient status as of the "statement covers through date"*/
    pub patient_status_code: Option<String>,
    /**CL1-04 (1345)
Code specifying the status of a nursing home resident at the time of service*/
    pub nursing_home_residential_status_code: Option<Fixed<1>>,
}
impl ClaimCodes {}