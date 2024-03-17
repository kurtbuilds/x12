use serde::{Serialize, Deserialize};
use super::super::elements::CompositeUnitOfMeasure;
use crate::fixed::Fixed;
/**To specify physical measurements or counts, including dimensions, tolerances, variances, and weights

(See Figures Appendix for example of use of C001)

See docs at <https://www.stedi.com/edi/x12/segment/MEA>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "MEA")]
pub struct Measurements {
    /**MEA-01 (737)
Code identifying the broad category to which a measurement applies*/
    pub measurement_reference_id_code: Option<Fixed<2>>,
    /**MEA-02 (738)
Code identifying a specific product or process characteristic to which a measurement applies*/
    pub measurement_qualifier: Option<String>,
    /**MEA-03 (739)
The value of the measurement*/
    pub measurement_value: Option<String>,
    /**MEA-04 (C001)
To identify a composite unit of measure

(See Figures Appendix for examples of use)*/
    pub composite_unit_of_measure: Option<CompositeUnitOfMeasure>,
    /**MEA-05 (740)
The value specifying the minimum of the measurement range*/
    pub range_minimum: Option<String>,
    /**MEA-06 (741)
The value specifying the maximum of the measurement range*/
    pub range_maximum: Option<String>,
    /**MEA-07 (935)
Code used to benchmark, qualify or further define a measurement value*/
    pub measurement_significance_code: Option<Fixed<2>>,
    /**MEA-08 (936)
Code used to express an attribute response when a numeric measurement value cannot be determined*/
    pub measurement_attribute_code: Option<Fixed<2>>,
    /**MEA-09 (752)
Code indicating the product surface, layer or position that is being described*/
    pub surface_layer_position_code: Option<Fixed<2>>,
    /**MEA-10 (1373)
The method or device used to record the measurement*/
    pub measurement_method_or_device: Option<String>,
    /**MEA-11 (1270)
Code identifying a specific industry code list*/
    pub code_list_qualifier_code: Option<String>,
    /**MEA-12 (1271)
Code indicating a code from a specific industry code list*/
    pub industry_code: Option<String>,
}
impl Measurements {}