use serde::{Serialize, Deserialize};
use super::super::codes::FacilityCodeQualifier;
use crate::fixed::Fixed;
/**To provide information that identifies the place of service or the type of bill related to the location at which a health care service was rendered

See docs at <https://www.stedi.com/edi/x12/element/C023>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C023")]
pub struct HealthCareServiceLocation {
    /**C023-01 (1331)
Code identifying where services were, or may be, performed; the first and second positions of the Uniform Bill Type Code for Institutional Services or the Place of Service Codes for Professional or Dental Services.*/
    pub health_care_service_location: String,
    /**C023-02 (1332)
Code identifying the type of facility referenced*/
    pub c023_02: Option<FacilityCodeQualifier>,
    /**C023-03 (1325)
Code specifying the frequency of the claim; this is the third position of the Uniform Billing Claim Form Bill Type*/
    pub c023_03: Option<Fixed<1>>,
}
impl HealthCareServiceLocation {}