use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To specify information associated with other health insurance coverage

See docs at <https://www.stedi.com/edi/x12/segment/OI>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "OI")]
pub struct OtherHealthInsurance {
    /**OI-01 (1032)
Code identifying type of claim*/
    pub claim_filing_indicator_code: Option<String>,
    /**OI-02 (1383)
Code identifying reason for claim submission*/
    pub claim_submission_reason_code: Option<Fixed<2>>,
    /**OI-03 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Option<Fixed<1>>,
    /**OI-04 (1351)
Code indicating how the patient or subscriber authorization signatures were obtained and how they are being retained by the provider*/
    pub patient_signature_source_code: Option<Fixed<1>>,
    /**OI-05 (1360)
Code indicating the type of agreement under which the provider is submitting this claim*/
    pub provider_agreement_code: Option<Fixed<1>>,
    /**OI-06 (1363)
Code indicating whether the provider has on file a signed statement by the patient authorizing the release of medical data to other organizations*/
    pub release_of_information_code: Option<Fixed<1>>,
    /**OI-07 (1359)
Code indicating whether the provider accepts assignment*/
    pub provider_accept_assignment_code: Option<Fixed<1>>,
    /**OI-08 (1073)
Code indicating a Yes or No condition or response*/
    pub oi08: Option<Fixed<1>>,
    /**OI-09 (1073)
Code indicating a Yes or No condition or response*/
    pub oi09: Option<Fixed<1>>,
    /**OI-10 (1073)
Code indicating a Yes or No condition or response*/
    pub oi10: Option<Fixed<1>>,
    /**OI-11 (1073)
Code indicating a Yes or No condition or response*/
    pub oi11: Option<Fixed<1>>,
}
impl OtherHealthInsurance {}