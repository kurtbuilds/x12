use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To identify the components of a Drug Use Review (DUR) alert

See docs at <https://www.stedi.com/edi/x12/element/C059>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C059")]
pub struct DrugUseReview {
    /**C059-01 (1738)
Code identifying the type of utilization conflict detected*/
    pub drug_use_review: Fixed<2>,
    /**C059-02 (1739)
Code identifying pharmacist intervention when a conflict code has been identified*/
    pub c059_02: Fixed<2>,
    /**C059-03 (1740)
Code identifying action taken by a pharmacist in response to a conflict*/
    pub c059_03: Fixed<2>,
}
impl DrugUseReview {}