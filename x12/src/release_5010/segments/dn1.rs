use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To supply orthodontic information

See docs at <https://www.stedi.com/edi/x12/segment/DN1>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "DN1")]
pub struct Orthodontic {
    /**DN1-01 (380)
Numeric value of quantity*/
    pub quantity: Option<String>,
    /**DN1-02 (380)
Numeric value of quantity*/
    pub dn1_02: Option<String>,
    /**DN1-03 (1073)
Code indicating a Yes or No condition or response*/
    pub yes_no_condition_or_response_code: Option<Fixed<1>>,
    /**DN1-04 (352)
A free-form description to clarify the related data elements and their content*/
    pub description: Option<String>,
}
impl Orthodontic {}