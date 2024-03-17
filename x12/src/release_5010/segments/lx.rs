use serde::{Serialize, Deserialize};
use crate::numeric::Numeric;
/**To reference a line number in a transaction set

See docs at <https://www.stedi.com/edi/x12/segment/LX>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "LX")]
pub struct TransactionSetLineNumber {
    /**LX-01 (554)
Number assigned for differentiation within a transaction set*/
    pub assigned_number: Numeric<0>,
}
impl TransactionSetLineNumber {}