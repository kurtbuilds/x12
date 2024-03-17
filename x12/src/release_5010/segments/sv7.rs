use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To specify the claim service detail for drug services that have been adjudicated

See docs at <https://www.stedi.com/edi/x12-005010/segment/SV7>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "SV7")]
pub struct DrugAdjudication {
    /**SV7-01 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub reference_identification: Option<String>,
    /**SV7-02 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub sv7_02: Option<String>,
    /**SV7-03 (1355)
Code indicating that a pharmacist has overridden the denial of a prescription*/
    pub prescription_denial_override_code: Option<Fixed<2>>,
    /**SV7-04 (1207)
Code indicating the level of coverage being provided for this insured*/
    pub coverage_level_code: Fixed<3>,
    /**SV7-05 (750)
Code identifying the general class of a product or process characteristic*/
    pub product_process_characteristic_code: String,
    /**SV7-06 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Option<Fixed<1>>,
}
impl DrugAdjudication {}