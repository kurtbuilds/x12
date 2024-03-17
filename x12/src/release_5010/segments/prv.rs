use serde::{Serialize, Deserialize};
use crate::fixed::Fixed;
use super::super::elements::ProviderSpecialty;
/**To specify the identifying characteristics of a provider

See docs at <https://www.stedi.com/edi/x12/segment/PRV>*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "PRV")]
pub struct Provider {
    /**PRV-01 (1221)
Code identifying the type of provider*/
    pub provider_code: String,
    /**PRV-02 (128)
Code qualifying the Reference Identification*/
    pub reference_identification_qualifier: Option<String>,
    /**PRV-03 (127)
Reference information as defined for a particular Transaction Set or as specified by the Reference Identification Qualifier*/
    pub reference_identification: Option<String>,
    /**PRV-04 (156)
Code (Standard State/Province) as defined by appropriate government agency*/
    pub state_or_province_code: Option<Fixed<2>>,
    /**PRV-05 (C035)
To provide provider specialty information*/
    pub provider_specialty: Option<ProviderSpecialty>,
    /**PRV-06 (1223)
Code identifying the organizational structure of a provider*/
    pub provider_organization_code: Option<Fixed<3>>,
}
impl Provider {}