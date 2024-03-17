use serde::{Serialize, Deserialize};
/**To identify standard industry codes

See docs at <https://www.stedi.com/edi/x12-005010/segment/LQ>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "LQ")]
pub struct IndustryCodeIdentification {
    /**LQ-01 (1270)
Code identifying a specific industry code list*/
    pub code_list_qualifier_code: Option<String>,
    /**LQ-02 (1271)
Code indicating a code from a specific industry code list*/
    pub industry_code: Option<String>,
}
impl IndustryCodeIdentification {}