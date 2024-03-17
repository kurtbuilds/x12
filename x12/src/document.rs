use serde::{Deserialize, Serialize};

use crate::{FunctionalGroupHeader, FunctionalGroupTrailer, InterchangeControlHeader, InterchangeControlTrailer};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Document<T> {
    /// ISA
    pub interchange_control_header: InterchangeControlHeader,
    /// GS
    pub functional_group_header: FunctionalGroupHeader,
    /// ST to SE
    pub transactions: Vec<T>,
    /// GE
    pub functional_group_trailer: FunctionalGroupTrailer,
    /// IEA
    pub interchange_control_trailer: InterchangeControlTrailer,
}