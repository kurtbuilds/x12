use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To indicate the start of a transaction set and to assign a control number

See docs at <https://www.stedi.com/edi/x12/segment/ST>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "ST")]
pub struct TransactionSetHeader {
    /**ST-01 (143)
Code uniquely identifying a Transaction Set*/
    pub transaction_set_identifier_code: Fixed<3>,
    /**ST-02 (329)
Identifying control number that must be unique within the transaction set functional group assigned by the originator for a transaction set*/
    pub transaction_set_control_number: String,
    /**ST-03 (1705)
Reference assigned to identify Implementation Convention*/
    pub implementation_convention_reference: Option<String>,
}
impl TransactionSetHeader {}