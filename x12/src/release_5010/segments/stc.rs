use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
use crate::release_5010::elements::HealthCareClaimStatus;
/**To report the status, required action, and paid information of a claim or service line

See docs at <https://www.stedi.com/edi/hipaa/segment/STC>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "STC")]
pub struct Status {
    /**STC-01 (C043)
Used to convey status of the entire claim or a specific service line*/
    pub health_care_claim_status: HealthCareClaimStatus,
    /**STC-02 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub date: Option<Fixed<8>>,
    /**STC-03 (306)
Code indicating type of action*/
    pub action_code: Option<String>,
    /**STC-04 (782)
Monetary amount*/
    pub monetary_amount: Option<String>,
    /**STC-05 (782)
Monetary amount*/
    pub stc05: Option<String>,
    /**STC-06 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub stc06: Option<Fixed<8>>,
    /**STC-07 (591)
Code identifying the method for the movement of payment instructions*/
    pub payment_method_code: Option<Fixed<3>>,
    /**STC-08 (373)
Date expressed as CCYYMMDD where CC represents the first two digits of the calendar year*/
    pub stc08: Option<Fixed<8>>,
    /**STC-09 (429)
Check identification number*/
    pub check_number: Option<String>,
    /**STC-10 (C043)
Used to convey status of the entire claim or a specific service line*/
    pub stc10: Option<HealthCareClaimStatus>,
    /**STC-11 (C043)
Used to convey status of the entire claim or a specific service line*/
    pub stc11: Option<HealthCareClaimStatus>,
    /**STC-12 (933)
Free-form message text*/
    pub free_form_message_text: Option<String>,
}
impl Status {}