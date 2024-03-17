use serde::{Serialize, Deserialize};
use super::super::codes::CodeListQualifierCode;
use crate::fixed::Fixed;
/**To send general and detailed information on race or ethnicity

See docs at <https://www.stedi.com/edi/x12/element/C056>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C056")]
pub struct CompositeRaceOrEthnicity {
    /**C056-01 (1109)
Code indicating the racial or ethnic background of a person; it is normally self-reported; Under certain circumstances this information is collected for United States Government statistical purposes*/
    pub composite_race_or_ethnicity: Option<Fixed<1>>,
    /**C056-02 (1270)
Code identifying a specific industry code list*/
    pub c056_02: Option<CodeListQualifierCode>,
    /**C056-03 (1271)
Code indicating a code from a specific industry code list*/
    pub c056_03: Option<String>,
}
impl CompositeRaceOrEthnicity {}