use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1359

See docs at <https://www.stedi.com/edi/x12/element/1359>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProviderAcceptAssignmentCode {
    ///A - Assigned
    Assigned,
    ///B - Assignment Accepted on Clinical Lab Services Only
    AssignmentAcceptedOnClinicalLabServicesOnly,
    ///C - Not Assigned
    NotAssigned,
    ///P - Patient Refuses to Assign Benefits
    PatientRefusesToAssignBenefits,
}
impl ProviderAcceptAssignmentCode {
    pub fn code(&self) -> &str {
        {
            use ProviderAcceptAssignmentCode::*;
            match self {
                Assigned => "A",
                AssignmentAcceptedOnClinicalLabServicesOnly => "B",
                NotAssigned => "C",
                PatientRefusesToAssignBenefits => "P",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ProviderAcceptAssignmentCode> {
        use ProviderAcceptAssignmentCode::*;
        match code {
            b"A" => Some(Assigned),
            b"B" => Some(AssignmentAcceptedOnClinicalLabServicesOnly),
            b"C" => Some(NotAssigned),
            b"P" => Some(PatientRefusesToAssignBenefits),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ProviderAcceptAssignmentCode::*;
        match self {
            Assigned => "Assigned",
            AssignmentAcceptedOnClinicalLabServicesOnly => {
                "Assignment Accepted on Clinical Lab Services Only"
            }
            NotAssigned => "Not Assigned",
            PatientRefusesToAssignBenefits => "Patient Refuses to Assign Benefits",
        }
    }
    fn from_description(description: &str) -> Option<ProviderAcceptAssignmentCode> {
        {
            use ProviderAcceptAssignmentCode::*;
            match description {
                "Assigned" => Some(Assigned),
                "Assignment Accepted on Clinical Lab Services Only" => {
                    Some(AssignmentAcceptedOnClinicalLabServicesOnly)
                }
                "Not Assigned" => Some(NotAssigned),
                "Patient Refuses to Assign Benefits" => {
                    Some(PatientRefusesToAssignBenefits)
                }
                _ => None,
            }
        }
    }
}
impl Serialize for ProviderAcceptAssignmentCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let value = if serializer.is_human_readable() {
            self.description()
        } else {
            self.code()
        };
        serializer.serialize_str(value)
    }
}
struct Visitor;
impl<'de> de::Visitor<'de> for Visitor {
    type Value = ProviderAcceptAssignmentCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Provider Accept Assignment Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ProviderAcceptAssignmentCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Provider Accept Assignment Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ProviderAcceptAssignmentCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Provider Accept Assignment Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ProviderAcceptAssignmentCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(Visitor)
        } else {
            deserializer.deserialize_bytes(Visitor)
        }
    }
}