use serde::{Serialize, Deserialize};
use super::super::elements::ToothSurface;
/**To identify a tooth by number and, if applicable, one or more tooth surfaces

See docs at <https://www.stedi.com/edi/x12/segment/TOO>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "TOO")]
pub struct ToothIdentification {
    /**TOO-01 (1270)
Code identifying a specific industry code list*/
    pub code_list_qualifier_code: Option<String>,
    /**TOO-02 (1271)
Code indicating a code from a specific industry code list*/
    pub industry_code: Option<String>,
    /**TOO-03 (C005)
To identify one or more tooth surface codes*/
    pub tooth_surface: Option<ToothSurface>,
}
impl ToothIdentification {}