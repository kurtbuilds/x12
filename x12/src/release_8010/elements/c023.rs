use serde::{Serialize, Deserialize};
use super::super::codes::FacilityCodeQualifier;
use crate::fixed::Fixed;
/**To provide information that identifies the place of service or the type of bill related to the location at which a health care service was rendered

See docs at <https://www.stedi.com/edi/x12/element/C023>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C023")]
pub struct HealthCareServiceLocation {
    /**C023-01 (1331)
Code identifying where services were, or may be, performed; the National Uniform Billing Committee (NUBC) Facility Type Code for Institutional Services or the Place of Service Codes for Professional or Dental Services.*/
    pub health_care_service_location: String,
    /**C023-02 (1332)
Code identifying the type of facility referenced*/
    pub c023_02: FacilityCodeQualifier,
    /**C023-03 (1325)
Code specifying the Type of Bill Frequency Code. It is the last digit of Type of Bill in the UB manual, as defined by the National Uniform Billing Committee*/
    pub c023_03: Option<Fixed<1>>,
}
impl HealthCareServiceLocation {}