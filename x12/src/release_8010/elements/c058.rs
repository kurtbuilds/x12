use serde::{Serialize, Deserialize};
use super::super::codes::CodeListQualifierCode;
/**To provide a reason and related explanation for a Health Care Claim or Service change in payment versus the original submitted charges

See docs at <https://www.stedi.com/edi/x12/element/C058>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C058")]
pub struct AdjustmentReason {
    /**C058-01 (1034)
Code identifying the detailed reason the adjustment was made*/
    pub adjustment_reason: String,
    /**C058-02 (1270)
Code identifying a specific industry code list*/
    pub c058_02: Option<CodeListQualifierCode>,
    /**C058-03 (1271)
Code indicating a code from a specific industry code list*/
    pub c058_03: Option<String>,
    /**C058-04 (1271)
Code indicating a code from a specific industry code list*/
    pub c058_04: Option<String>,
    /**C058-05 (1271)
Code indicating a code from a specific industry code list*/
    pub c058_05: Option<String>,
    /**C058-06 (1271)
Code indicating a code from a specific industry code list*/
    pub c058_06: Option<String>,
    /**C058-07 (1271)
Code indicating a code from a specific industry code list*/
    pub c058_07: Option<String>,
}
impl AdjustmentReason {}