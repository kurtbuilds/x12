use serde::{Serialize, Deserialize};
use super::super::elements::CompositeUnitOfMeasure;
use crate::fixed::Fixed;
/**To specify quantity information

See docs at <https://www.stedi.com/edi/x12/segment/QTY>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "QTY")]
pub struct Quantity {
    /**QTY-01 (673)
Code specifying the type of quantity*/
    pub quantity_qualifier: Fixed<2>,
    /**QTY-02 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**QTY-03 (C001)
To identify a composite unit of measure

(See Figures Appendix for examples of use)*/
    pub composite_unit_of_measure: Option<CompositeUnitOfMeasure>,
    /**QTY-04 (61)
Free-form information*/
    pub free_form: Option<String>,
}
impl Quantity {}