use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To supply information specific to hospital claims

See docs at <https://www.stedi.com/edi/x12/segment/CL1>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "CL1")]
pub struct ClaimCodes {
    /**CL1-01 (1315)
A code indicating the priority of this admission/visit.*/
    pub priority: Option<Fixed<1>>,
    /**CL1-02 (1314)
A code indicating the point of patient origin for this admission or visit.*/
    pub point_of_origin_for_admission_or_visit: Option<Fixed<1>>,
    /**CL1-03 (1352)
A code indicating the disposition or discharge status of the patient as of the discharge date.*/
    pub patient_discharge_status: Option<String>,
    /**CL1-04 (1345)
Code specifying the status of a nursing home resident at the time of service*/
    pub nursing_home_residential_status_code: Option<Fixed<1>>,
}
impl ClaimCodes {}