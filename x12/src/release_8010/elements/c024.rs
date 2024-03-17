use serde::{Serialize, Deserialize};
use super::super::codes::CountryCode;
use super::super::codes::RelatedCausesCode;
use crate::fixed::Fixed;
/**To identify one or more related causes and associated state or country information

See docs at <https://www.stedi.com/edi/x12/element/C024>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C024")]
pub struct RelatedCauses {
    /**C024-01 (1362)
Code identifying an accompanying cause of an illness, injury or an accident*/
    pub related_causes: RelatedCausesCode,
    /**C024-02 (1362)
Code identifying an accompanying cause of an illness, injury or an accident*/
    pub c024_02: Option<RelatedCausesCode>,
    /**C024-03 (1362)
Code identifying an accompanying cause of an illness, injury or an accident*/
    pub c024_03: Option<RelatedCausesCode>,
    /**C024-04 (156)
Code specifying the Standard State/Province as defined by appropriate government agency*/
    pub c024_04: Option<Fixed<2>>,
    /**C024-05 (26)
Code identifying the country*/
    pub c024_05: Option<CountryCode>,
}
impl RelatedCauses {}