use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To record information specific to the primary insured and the insurance carrier for that insured

See docs at <https://www.stedi.com/edi/x12-005010/segment/SBR>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "SBR")]
pub struct Subscriber {
    /**SBR-01 (1138)
Code identifying the insurance carrier's level of responsibility for a payment of a claim*/
    pub payer_responsibility_sequence_number_code: Fixed<1>,
    /**SBR-02 (1069)
Code indicating the relationship between two individuals or entities*/
    pub individual_relationship_code: Option<Fixed<2>>,
    /**SBR-03 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub reference_identification: Option<String>,
    /**SBR-04 (93)
Free-form name*/
    pub name: Option<String>,
    /**SBR-05 (1336)
Code identifying the type of insurance policy within a specific insurance program*/
    pub insurance_type_code: Option<String>,
    /**SBR-06 (1143)
Code identifying whether there is a coordination of benefits*/
    pub coordination_of_benefits_code: Option<Fixed<1>>,
    /**SBR-07 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Option<Fixed<1>>,
    /**SBR-08 (584)
Code showing the general employment status of an employee/claimant*/
    pub employment_status_code: Option<Fixed<2>>,
    /**SBR-09 (1032)
Code identifying type of claim*/
    pub claim_filing_indicator_code: Option<String>,
}
impl Subscriber {}