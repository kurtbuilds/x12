use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To supply information regarding certification of medical necessity for home oxygen therapy

See docs at <https://www.stedi.com/edi/x12/segment/CR5>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "CR5")]
pub struct OxygenTherapyCertification {
    /**CR5-01 (1322)
Code indicating the type of certification*/
    pub certification_type_code: Option<Fixed<1>>,
    /**CR5-02 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**CR5-03 (1348)
Code indicating the specific type of equipment being prescribed for the delivery of oxygen*/
    pub oxygen_equipment_type_code: Option<Fixed<1>>,
    /**CR5-04 (1348)
Code indicating the specific type of equipment being prescribed for the delivery of oxygen*/
    pub cr5_04: Option<Fixed<1>>,
    /**CR5-05 (352)
A free-form description to clarify the related data elements and their content*/
    pub description: Option<String>,
    /**CR5-06 (380)
Numeric value of quantity*/
    pub cr5_06: Option<String>,
    /**CR5-07 (380)
Numeric value of quantity*/
    pub cr5_07: Option<String>,
    /**CR5-08 (380)
Numeric value of quantity*/
    pub cr5_08: Option<String>,
    /**CR5-09 (352)
A free-form description to clarify the related data elements and their content*/
    pub cr5_09: Option<String>,
    /**CR5-10 (380)
Numeric value of quantity*/
    pub cr5_10: Option<String>,
    /**CR5-11 (380)
Numeric value of quantity*/
    pub cr5_11: Option<String>,
    /**CR5-12 (1349)
Code indicating the conditions under which a patient was tested*/
    pub oxygen_test_condition_code: Option<Fixed<1>>,
    /**CR5-13 (1350)
Code indicating the findings of oxygen tests performed on a patient*/
    pub oxygen_test_findings_code: Option<Fixed<1>>,
    /**CR5-14 (1350)
Code indicating the findings of oxygen tests performed on a patient*/
    pub cr5_14: Option<Fixed<1>>,
    /**CR5-15 (1350)
Code indicating the findings of oxygen tests performed on a patient*/
    pub cr5_15: Option<Fixed<1>>,
    /**CR5-16 (380)
Numeric value of quantity*/
    pub cr5_16: Option<String>,
    /**CR5-17 (1382)
Code to indicate if a particular form of delivery was prescribed*/
    pub oxygen_delivery_system_code: Option<Fixed<1>>,
    /**CR5-18 (1348)
Code indicating the specific type of equipment being prescribed for the delivery of oxygen*/
    pub cr5_18: Option<Fixed<1>>,
}
impl OxygenTherapyCertification {}