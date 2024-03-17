use serde::{Serialize, Deserialize};
use super::super::codes::ReferenceIdentificationQualifier;
/**To identify one or more reference numbers or identification numbers as specified by the Reference Qualifier

See docs at <https://www.stedi.com/edi/x12/element/C040>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C040")]
pub struct Reference {
    /**C040-01 (128)
Code identifying the Reference Identification*/
    pub reference: ReferenceIdentificationQualifier,
    /**C040-02 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub c040_02: String,
    /**C040-03 (128)
Code identifying the Reference Identification*/
    pub c040_03: Option<ReferenceIdentificationQualifier>,
    /**C040-04 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub c040_04: Option<String>,
    /**C040-05 (128)
Code identifying the Reference Identification*/
    pub c040_05: Option<ReferenceIdentificationQualifier>,
    /**C040-06 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub c040_06: Option<String>,
}
impl Reference {}