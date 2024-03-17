use serde::{Serialize, Deserialize};
/**To specify the location of the named party

See docs at <https://www.stedi.com/edi/x12-005010/segment/N3>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "N3")]
pub struct PartyLocation {
    /**N3-01 (166)
Address information*/
    pub address: String,
    /**N3-02 (166)
Address information*/
    pub n3_02: Option<String>,
}
impl PartyLocation {}