use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To supply disability information

See docs at <https://www.stedi.com/edi/x12/segment/DSB>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "DSB")]
pub struct Disability {
    /**DSB-01 (1146)
Code identifying the disability status of the individual*/
    pub disability_type_code: Fixed<1>,
    /**DSB-02 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**DSB-03 (1149)
Code identifying the occupation of an individual*/
    pub occupation_code: Option<String>,
    /**DSB-04 (1154)
Code identifying the level of intensity of work*/
    pub work_intensity_code: Option<Fixed<1>>,
    /**DSB-05 (1161)
Code indicating an option chosen for the product*/
    pub product_option_code: Option<String>,
    /**DSB-06 (782)
Monetary amount*/
    pub monetary_amount: Option<String>,
    /**DSB-07 (235)
Code identifying the type/source of the descriptive number used in Product/Service ID (234)*/
    pub product_service_id_qualifier: Option<Fixed<2>>,
    /**DSB-08 (1137)
Code value for describing a medical condition or procedure*/
    pub medical_code_value: Option<String>,
}
impl Disability {}