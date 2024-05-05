use serde::{Serialize, Deserialize};
use super::super::codes::CodeListQualifierCode;
use super::super::codes::EntityIdentifierCode;
/**Used to convey status of the entire claim or a specific service line

See docs at <https://www.stedi.com/edi/hipaa/element/C043>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C043")]
pub struct HealthCareClaimStatus {
    /**C043-01 (1271)
Code indicating a code from a specific industry code list*/
    pub health_care_claim_status: String,
    /**C043-02 (1271)
Code indicating a code from a specific industry code list*/
    pub c043_02: String,
    /**C043-03 (98)
Code identifying an organizational entity, a physical location, property or an individual*/
    pub c043_03: Option<EntityIdentifierCode>,
    /**C043-04 (1270)
Code identifying a specific industry code list*/
    pub c043_04: Option<CodeListQualifierCode>,
}
impl HealthCareClaimStatus {}