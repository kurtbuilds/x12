use serde::{Serialize, Deserialize};
use crate::numeric::Numeric;
/**To indicate the end of the transaction set and provide the count of the transmitted segments (including the beginning (ST) and ending (SE) segments)

See docs at <https://www.stedi.com/edi/x12/segment/SE>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "SE")]
pub struct TransactionSetTrailer {
    /**SE-01 (96)
Total number of segments included in a transaction set including ST and SE segments*/
    pub number_of_included_segments: Numeric<0>,
    /**SE-02 (329)
Identifying control number that must be unique within the transaction set functional group assigned by the originator for a transaction set*/
    pub transaction_set_control_number: String,
}
impl TransactionSetTrailer {}