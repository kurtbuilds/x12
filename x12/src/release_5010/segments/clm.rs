use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
use super::super::elements::RelatedCauses;
use super::super::elements::HealthCareServiceLocation;
/**To specify basic data about the claim

See docs at <https://www.stedi.com/edi/x12-005010/segment/CLM>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "CLM")]
pub struct HealthClaim {
    /**CLM-01 (1028)
Identifier used to track a claim from creation by the health care provider through payment*/
    pub claim_submitters: String,
    /**CLM-02 (782)
Monetary amount*/
    pub monetary_amount: Option<String>,
    /**CLM-03 (1032)
Code identifying type of claim*/
    pub claim_filing_indicator_code: Option<String>,
    /**CLM-04 (1343)
Code identifying the type of provider or claim*/
    pub non_institutional_claim_type_code: Option<String>,
    /**CLM-05 (C023)
To provide information that identifies the place of service or the type of bill related to the location at which a health care service was rendered*/
    pub health_care_service_location: Option<HealthCareServiceLocation>,
    /**CLM-06 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Option<Fixed<1>>,
    /**CLM-07 (1359)
Code indicating whether the provider accepts assignment*/
    pub provider_accept_assignment_code: Option<Fixed<1>>,
    /**CLM-08 (1073)
Code indicating a Yes or No condition or response*/
    pub clm08: Option<Fixed<1>>,
    /**CLM-09 (1363)
Code indicating whether the provider has on file a signed statement by the patient authorizing the release of medical data to other organizations*/
    pub release_of_information_code: Option<Fixed<1>>,
    /**CLM-10 (1351)
Code indicating how the patient or subscriber authorization signatures were obtained and how they are being retained by the provider*/
    pub patient_signature_source_code: Option<Fixed<1>>,
    /**CLM-11 (C024)
To identify one or more related causes and associated state or country information*/
    pub related_causes: Option<RelatedCauses>,
    /**CLM-12 (1366)
Code indicating the Special Program under which the services rendered to the patient were performed*/
    pub special_program_code: Option<String>,
    /**CLM-13 (1073)
Code indicating a Yes or No condition or response*/
    pub clm13: Option<Fixed<1>>,
    /**CLM-14 (1338)
Code specifying the level of service rendered*/
    pub level_of_service_code: Option<String>,
    /**CLM-15 (1073)
Code indicating a Yes or No condition or response*/
    pub clm15: Option<Fixed<1>>,
    /**CLM-16 (1360)
Code indicating the type of agreement under which the provider is submitting this claim*/
    pub provider_agreement_code: Option<Fixed<1>>,
    /**CLM-17 (1029)
Code identifying the status of an entire claim as assigned by the payor, claim review organization or repricing organization*/
    pub claim_status_code: Option<String>,
    /**CLM-18 (1073)
Code indicating a Yes or No condition or response*/
    pub clm18: Option<Fixed<1>>,
    /**CLM-19 (1383)
Code identifying reason for claim submission*/
    pub claim_submission_reason_code: Option<Fixed<2>>,
    /**CLM-20 (1514)
Code indicating the reason why a request was delayed*/
    pub delay_reason_code: Option<String>,
}
impl HealthClaim {}