use serde::{Deserialize, Serialize};
use crate::Numeric;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "GE")]
pub struct FunctionalGroupTrailer {
    pub number_of_included_transaction_sets: Numeric<0>,
    pub group_control_number: Numeric<0>,
}
