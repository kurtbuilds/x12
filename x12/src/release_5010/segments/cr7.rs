use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
use crate::numeric::Numeric;
/**To supply information related to the home health care plan of treatment and services

See docs at <https://www.stedi.com/edi/x12-005010/segment/CR7>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "CR7")]
pub struct HomeHealthTreatmentPlanCertification {
    /**CR7-01 (921)
Code indicating disciplines ordered by a physician*/
    pub discipline_type_code: Fixed<2>,
    /**CR7-02 (1470)
A generic number*/
    pub number: Numeric<0>,
    /**CR7-03 (1470)
A generic number*/
    pub cr7_03: Numeric<0>,
}
impl HomeHealthTreatmentPlanCertification {}