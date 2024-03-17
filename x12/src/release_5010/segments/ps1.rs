use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To specify the information about services that are purchased

See docs at <https://www.stedi.com/edi/x12/segment/PS1>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "PS1")]
pub struct PurchaseService {
    /**PS1-01 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub reference_identification: String,
    /**PS1-02 (782)
Monetary amount*/
    pub monetary_amount: String,
    /**PS1-03 (156)
Code (Standard State/Province) as defined by appropriate government agency*/
    pub state_or_province_code: Option<Fixed<2>>,
}
impl PurchaseService {}