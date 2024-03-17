use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**Used to establish a proposed value for a trade item for marketing purposes

See docs at <https://www.stedi.com/edi/x12/element/C077>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C077")]
pub struct CompositeCurrency {
    /**C077-01 (212)
Price per unit of product, service, commodity, etc.*/
    pub composite_currency: String,
    /**C077-02 (100)
Code specifying the Standard ISO code for country in whose currency the charges are specified*/
    pub c077_02: Fixed<3>,
    /**C077-03 (212)
Price per unit of product, service, commodity, etc.*/
    pub c077_03: Option<String>,
    /**C077-04 (100)
Code specifying the Standard ISO code for country in whose currency the charges are specified*/
    pub c077_04: Option<Fixed<3>>,
    /**C077-05 (212)
Price per unit of product, service, commodity, etc.*/
    pub c077_05: Option<String>,
    /**C077-06 (100)
Code specifying the Standard ISO code for country in whose currency the charges are specified*/
    pub c077_06: Option<Fixed<3>>,
}
impl CompositeCurrency {}