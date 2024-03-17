use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To indicate the total monetary amount

See docs at <https://www.stedi.com/edi/x12-005010/segment/AMT>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "AMT")]
pub struct MonetaryAmount {
    /**AMT-01 (522)
Code to qualify amount*/
    pub amount_qualifier_code: String,
    /**AMT-02 (782)
Monetary amount*/
    pub monetary_amount: String,
    /**AMT-03 (478)
Code indicating whether amount is a credit or debit*/
    pub credit_debit_flag_code: Option<Fixed<1>>,
}
impl MonetaryAmount {}