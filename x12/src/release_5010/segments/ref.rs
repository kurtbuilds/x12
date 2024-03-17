use serde::{Serialize, Deserialize};
use super::super::elements;
/**To specify identifying information

See docs at <https://www.stedi.com/edi/x12-005010/segment/REF>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "REF")]
pub struct Reference {
    /**REF-01 (128)
Code qualifying the Reference Identification*/
    pub reference_identification_qualifier: String,
    /**REF-02 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub reference_identification: Option<String>,
    /**REF-03 (352)
A free-form description to clarify the related data elements and their content*/
    pub description: Option<String>,
    /**REF-04 (C040)
To identify one or more reference numbers or identification numbers as specified by the Reference Qualifier*/
    pub reference: Option<elements::Reference>,
}
impl Reference {}