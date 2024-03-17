use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "code", rename = "GS")]
pub struct FunctionalGroupHeader {
    pub identifier_code: String,
    pub sender_code: String,
    pub receiver_code: String,
    pub date: String,
    pub time: String,
    pub control_number: String,
    pub responsible_agency_code: String,
    pub version: String,
}
