use serde::{Deserialize, Serialize};
use crate::Numeric;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "IEA")]
pub struct InterchangeControlTrailer {
    pub number_of_included_functional_groups: Numeric<0>,
    pub interchange_control_number: Numeric<0>,
}
