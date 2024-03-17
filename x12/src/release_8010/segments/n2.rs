use serde::{Serialize, Deserialize};
/**To specify additional names

See docs at <https://www.stedi.com/edi/x12/segment/N2>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "N2")]
pub struct AdditionalName {
    /**N2-01 (93)
Free-form name*/
    pub name: String,
    /**N2-02 (93)
Free-form name*/
    pub n2_02: Option<String>,
}
impl AdditionalName {}