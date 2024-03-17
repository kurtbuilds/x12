use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To specify language, type of usage, and proficiency or fluency

See docs at <https://www.stedi.com/edi/x12/segment/LUI>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "LUI")]
pub struct LanguageUse {
    /**LUI-01 (66)
Code specifying the system/method of code structure used for Identification Code (67)*/
    pub identification_code_qualifier: Option<String>,
    /**LUI-02 (67)
Code identifying a party or other code*/
    pub identification_code: Option<String>,
    /**LUI-03 (352)
A free-form description to clarify the related data elements and their content*/
    pub description: Option<String>,
    /**LUI-04 (1303)
Code indicating the use of a language*/
    pub use_of_language_indicator_code: Option<String>,
    /**LUI-05 (1476)
Code indicating language proficiency*/
    pub language_proficiency_indicator_code: Option<Fixed<1>>,
}
impl LanguageUse {}