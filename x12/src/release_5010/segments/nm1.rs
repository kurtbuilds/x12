use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
/**To supply the full name of an individual or organizational entity

See docs at <https://www.stedi.com/edi/x12/segment/NM1>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "NM1")]
pub struct IndividualOrOrganizationalName {
    /**NM1-01 (98)
Code identifying an organizational entity, a physical location, property or an individual*/
    pub entity_identifier_code: String,
    /**NM1-02 (1065)
Code qualifying the type of entity*/
    pub entity_type_qualifier: Fixed<1>,
    /**NM1-03 (1035)
Individual last name or organizational name*/
    pub name_last_or_organization_name: Option<String>,
    /**NM1-04 (1036)
Individual first name*/
    pub name_first: Option<String>,
    /**NM1-05 (1037)
Individual middle name or initial*/
    pub name_middle: Option<String>,
    /**NM1-06 (1038)
Prefix to individual name*/
    pub name_prefix: Option<String>,
    /**NM1-07 (1039)
Suffix to individual name*/
    pub name_suffix: Option<String>,
    /**NM1-08 (66)
Code designating the system/method of code structure used for Identification Code (67)*/
    pub identification_code_qualifier: Option<String>,
    /**NM1-09 (67)
Code identifying a party or other code*/
    pub identification_code: Option<String>,
    /**NM1-10 (706)
Code describing entity relationship*/
    pub entity_relationship_code: Option<Fixed<2>>,
    /**NM1-11 (98)
Code identifying an organizational entity, a physical location, property or an individual*/
    pub nm1_11: Option<String>,
    /**NM1-12 (1035)
Individual last name or organizational name*/
    pub nm1_12: Option<String>,
}
impl IndividualOrOrganizationalName {}