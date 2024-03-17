use serde::{Serialize, Deserialize};
/**To indicate that the loop immediately preceding this segment is complete

See docs at <https://www.stedi.com/edi/x12/segment/LE>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "LE")]
pub struct LoopTrailer {
    /**LE-01 (447)
The loop ID number given on the transaction set diagram is the value for this data element in segments LS and LE*/
    pub loop_identifier_code: String,
}
impl LoopTrailer {}