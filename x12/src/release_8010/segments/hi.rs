use serde::{Serialize, Deserialize};
use super::super::elements::HealthCareCode;
/**To supply information related to the delivery of health care

See docs at <https://www.stedi.com/edi/x12/segment/HI>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "HI")]
pub struct HealthCareInformationCodes {
    /**HI-01 (C022)
To send health care codes and their associated dates, amounts and quantities*/
    pub health_care_code: Vec<HealthCareCode>,
}
impl HealthCareInformationCodes {}