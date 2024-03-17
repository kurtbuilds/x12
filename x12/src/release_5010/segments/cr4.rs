use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To supply information regarding certification of medical necessity for enteral or parenteral nutrition therapy

See docs at <https://www.stedi.com/edi/x12-005010/segment/CR4>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "CR4")]
pub struct EnteralOrParenteralTherapyCertification {
    /**CR4-01 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Fixed<1>,
    /**CR4-02 (1322)
Code indicating the type of certification*/
    pub certification_type_code: Option<Fixed<1>>,
    /**CR4-03 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub unit_or_basis_for_measurement_code: Option<Fixed<2>>,
    /**CR4-04 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**CR4-05 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub cr4_05: Option<Fixed<2>>,
    /**CR4-06 (380)
Numeric value of quantity*/
    pub cr4_06: Option<String>,
    /**CR4-07 (1344)
Code indicating on what a medical evaluation is based, when not based on a physician's visit*/
    pub non_visit_code: Option<Fixed<1>>,
    /**CR4-08 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub cr4_08: Option<Fixed<2>>,
    /**CR4-09 (380)
Numeric value of quantity*/
    pub cr4_09: Option<String>,
    /**CR4-10 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub cr4_10: Option<Fixed<2>>,
    /**CR4-11 (65)
Vertical dimension of an object measured when the object is in the upright position*/
    pub height: Option<String>,
    /**CR4-12 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub cr4_12: Option<Fixed<2>>,
    /**CR4-13 (81)
Numeric value of weight*/
    pub weight: Option<String>,
    /**CR4-14 (380)
Numeric value of quantity*/
    pub cr4_14: Option<String>,
    /**CR4-15 (352)
A free-form description to clarify the related data elements and their content*/
    pub description: Option<String>,
    /**CR4-16 (1346)
Code indicating the method used to administer nutrient to a patient*/
    pub nutrient_administration_method_code: Option<Fixed<1>>,
    /**CR4-17 (1347)
Code indicating the technique used to administer nutrient to a patient*/
    pub nutrient_administration_technique_code: Option<Fixed<1>>,
    /**CR4-18 (380)
Numeric value of quantity*/
    pub cr4_18: Option<String>,
    /**CR4-19 (380)
Numeric value of quantity*/
    pub cr4_19: Option<String>,
    /**CR4-20 (352)
A free-form description to clarify the related data elements and their content*/
    pub cr4_20: Option<String>,
    /**CR4-21 (380)
Numeric value of quantity*/
    pub cr4_21: Option<String>,
    /**CR4-22 (954)
Percentage expressed as a decimal (e.g., 0.0 through 1.0 represents 0% through 100%)*/
    pub percentage_as_decimal: Option<String>,
    /**CR4-23 (380)
Numeric value of quantity*/
    pub cr4_23: Option<String>,
    /**CR4-24 (380)
Numeric value of quantity*/
    pub cr4_24: Option<String>,
    /**CR4-25 (954)
Percentage expressed as a decimal (e.g., 0.0 through 1.0 represents 0% through 100%)*/
    pub cr4_25: Option<String>,
    /**CR4-26 (380)
Numeric value of quantity*/
    pub cr4_26: Option<String>,
    /**CR4-27 (954)
Percentage expressed as a decimal (e.g., 0.0 through 1.0 represents 0% through 100%)*/
    pub cr4_27: Option<String>,
    /**CR4-28 (380)
Numeric value of quantity*/
    pub cr4_28: Option<String>,
    /**CR4-29 (352)
A free-form description to clarify the related data elements and their content*/
    pub cr4_29: Option<String>,
}
impl EnteralOrParenteralTherapyCertification {}