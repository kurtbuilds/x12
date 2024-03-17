use serde::{Serialize, Deserialize};
use super::super::codes::ProviderSpecialtyCode;
use crate::fixed::Fixed;
/**To provide provider specialty information

See docs at <https://www.stedi.com/edi/x12-005010/element/C035>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "C035")]
pub struct ProviderSpecialty {
    /**C035-01 (1222)
Code indicating the primary specialty of the provider, as defined by the receiver*/
    pub provider_specialty: ProviderSpecialtyCode,
    /**C035-02 (559)
Code identifying the agency assigning the code values*/
    pub c035_02: Option<Fixed<2>>,
    /**C035-03 (1073)
Code indicating a Yes or No condition or response*/
    pub c035_03: Option<Fixed<1>>,
}
impl ProviderSpecialty {}