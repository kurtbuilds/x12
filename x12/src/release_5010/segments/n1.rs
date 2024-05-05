use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To identify a party by type of organization, name, and code

See docs at <https://www.stedi.com/edi/release_5010/segment/N1>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "N1")]
pub struct PartyIdentification {
    /**N1-01 (98)
Code identifying an organizational entity, a physical location, property or an individual*/
    pub entity_identifier_code: String,
    /**N1-02 (93)
Free-form name*/
    pub name: Option<String>,
    /**N1-03 (66)
Code designating the system/method of code structure used for Identification Code (67)*/
    pub identification_code_qualifier: Option<String>,
    /**N1-04 (67)
Code identifying a party or other code*/
    pub identification_code: Option<String>,
    /**N1-05 (706)
Code describing entity relationship*/
    pub entity_relationship_code: Option<Fixed<2>>,
    /**N1-06 (98)
Code identifying an organizational entity, a physical location, property or an individual*/
    pub n1_06: Option<String>,
}
impl PartyIdentification {}