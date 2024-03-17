use serde::{Serialize, Deserialize};
use crate::numeric::Numeric;
use crate::fixed::Fixed;
/**To supply information related to the chiropractic service rendered to a patient

See docs at <https://www.stedi.com/edi/x12-005010/segment/CR2>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "CR2")]
pub struct ChiropracticCertification {
    /**CR2-01 (609)
Occurrence counter*/
    pub count: Option<Numeric<0>>,
    /**CR2-02 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**CR2-03 (1367)
Code identifying the specific level of subluxation*/
    pub subluxation_level_code: Option<String>,
    /**CR2-04 (1367)
Code identifying the specific level of subluxation*/
    pub cr2_04: Option<String>,
    /**CR2-05 (355)
Code specifying the units in which a value is being expressed, or manner in which a measurement has been taken*/
    pub unit_or_basis_for_measurement_code: Option<Fixed<2>>,
    /**CR2-06 (380)
Numeric value of quantity*/
    pub cr2_06: Option<String>,
    /**CR2-07 (380)
Numeric value of quantity*/
    pub cr2_07: Option<String>,
    /**CR2-08 (1342)
Code indicating the nature of a patient's condition*/
    pub nature_of_condition_code: Option<Fixed<1>>,
    /**CR2-09 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Option<Fixed<1>>,
    /**CR2-10 (352)
A free-form description to clarify the related data elements and their content*/
    pub description: Option<String>,
    /**CR2-11 (352)
A free-form description to clarify the related data elements and their content*/
    pub cr2_11: Option<String>,
    /**CR2-12 (1073)
Code indicating a Yes or No condition or response*/
    pub cr2_12: Option<Fixed<1>>,
}
impl ChiropracticCertification {}