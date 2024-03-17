use serde::{Serialize, Deserialize};
/**To indicate that the next segment begins a loop

See docs at <https://www.stedi.com/edi/x12/segment/LS>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "LS")]
pub struct LoopHeader {
    /**LS-01 (447)
The loop ID number given on the transaction set diagram is the value for this data element in segments LS and LE*/
    pub loop_identifier_code: String,
}
impl LoopHeader {}