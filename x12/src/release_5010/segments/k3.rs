use serde::{Serialize, Deserialize};
use super::super::elements::CompositeUnitOfMeasure;
/**To transmit a fixed-format record or matrix contents

See docs at <https://www.stedi.com/edi/x12/segment/K3>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "K3")]
pub struct File {
    /**K3-01 (449)
Data in fixed format agreed upon by sender and receiver*/
    pub fixed_format: String,
    /**K3-02 (1333)
Code specifying the format of information*/
    pub record_format_code: Option<String>,
    /**K3-03 (C001)
To identify a composite unit of measure

(See Figures Appendix for examples of use)*/
    pub composite_unit_of_measure: Option<CompositeUnitOfMeasure>,
}
impl File {}