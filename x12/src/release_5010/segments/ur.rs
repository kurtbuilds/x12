use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To specify the results of the utilization review

See docs at <https://www.stedi.com/edi/x12/segment/UR>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "UR")]
pub struct PeerReviewOrganizationOrUtilizationReview {
    /**UR-01 (1318)
Code specifying the determination arrived at by the Utilization Review Committee or Peer Review Organization (PRO)*/
    pub approval_code: Fixed<1>,
    /**UR-02 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
}
impl PeerReviewOrganizationOrUtilizationReview {}