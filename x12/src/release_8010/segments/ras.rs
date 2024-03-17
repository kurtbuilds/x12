use serde::{Serialize, Deserialize};
use super::super::elements::AdjustmentReason;
/**To supply Claim Adjustment Reason Codes and amounts as needed for an entire claim or for a particular service within the claim being paid

See docs at <https://www.stedi.com/edi/x12/segment/RAS>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "RAS")]
pub struct ReasonAdjustment {
    /**RAS-01 (782)
Monetary amount*/
    pub monetary_amount: String,
    /**RAS-02 (1785)
Code identifying the general category of payment adjustment.*/
    pub claim_adjustment_group_code: String,
    /**RAS-03 (C058)
To provide a reason and related explanation for a Health Care Claim or Service change in payment versus the original submitted charges*/
    pub adjustment_reason: Vec<AdjustmentReason>,
    /**RAS-04 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
}
impl ReasonAdjustment {}