use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To specify basic data about the contract or contract line item

See docs at <https://www.stedi.com/edi/x12/segment/CN1>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "CN1")]
pub struct Contract {
    /**CN1-01 (1166)
Code identifying a contract type*/
    pub contract_type_code: Fixed<2>,
    /**CN1-02 (782)
Monetary amount*/
    pub monetary_amount: Option<String>,
    /**CN1-03 (332)
Percent given in decimal format (e.g., 0.0 through 100.0 represents 0% through 100%)*/
    pub percent: Option<String>,
    /**CN1-04 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub reference_identification: Option<String>,
    /**CN1-05 (338)
Terms discount percentage, expressed as a percent, available to the purchaser if an invoice is paid on or before the Terms Discount Due Date*/
    pub terms_discount_percent: Option<String>,
    /**CN1-06 (799)
Revision level of a particular format, program, technique or algorithm*/
    pub version: Option<String>,
}
impl Contract {}