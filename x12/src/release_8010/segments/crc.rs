use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To supply information on conditions

See docs at <https://www.stedi.com/edi/x12/segment/CRC>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "CRC")]
pub struct ConditionsIndicator {
    /**CRC-01 (1136)
Code specifying the situation or category to which the code applies*/
    pub code_category: Fixed<2>,
    /**CRC-02 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Fixed<1>,
    /**CRC-03 (1321)
Code indicating a condition*/
    pub condition_indicator_code: String,
    /**CRC-04 (1321)
Code indicating a condition*/
    pub crc04: Option<String>,
    /**CRC-05 (1321)
Code indicating a condition*/
    pub crc05: Option<String>,
    /**CRC-06 (1321)
Code indicating a condition*/
    pub crc06: Option<String>,
    /**CRC-07 (1321)
Code indicating a condition*/
    pub crc07: Option<String>,
}
impl ConditionsIndicator {}