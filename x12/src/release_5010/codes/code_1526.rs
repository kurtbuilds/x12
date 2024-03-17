use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1526

See docs at <https://www.stedi.com/edi/x12-005010/element/1526>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PolicyComplianceCode {
    ///1 - Procedure Followed (Compliance)
    Code1,
    ///2 - Not Followed - Call Not Made (Non-Compliance Call Not Made)
    Code2,
    ///3 - Not Medically Necessary (Non-Compliance Non-Medically Necessary)
    Code3,
    ///4 - Not Followed Other (Non-Compliance Other)
    Code4,
    ///5 - Emergency Admit to Non-Network Hospital
    EmergencyAdmitToNonNetworkHospital,
}
impl PolicyComplianceCode {
    pub fn code(&self) -> &str {
        {
            use PolicyComplianceCode::*;
            match self {
                Code1 => "1",
                Code2 => "2",
                Code3 => "3",
                Code4 => "4",
                EmergencyAdmitToNonNetworkHospital => "5",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<PolicyComplianceCode> {
        use PolicyComplianceCode::*;
        match code {
            b"1" => Some(Code1),
            b"2" => Some(Code2),
            b"3" => Some(Code3),
            b"4" => Some(Code4),
            b"5" => Some(EmergencyAdmitToNonNetworkHospital),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use PolicyComplianceCode::*;
        match self {
            Code1 => "Procedure Followed (Compliance)",
            Code2 => "Not Followed - Call Not Made (Non-Compliance Call Not Made)",
            Code3 => "Not Medically Necessary (Non-Compliance Non-Medically Necessary)",
            Code4 => "Not Followed Other (Non-Compliance Other)",
            EmergencyAdmitToNonNetworkHospital => {
                "Emergency Admit to Non-Network Hospital"
            }
        }
    }
    fn from_description(description: &str) -> Option<PolicyComplianceCode> {
        {
            use PolicyComplianceCode::*;
            match description {
                "Procedure Followed (Compliance)" => Some(Code1),
                "Not Followed - Call Not Made (Non-Compliance Call Not Made)" => {
                    Some(Code2)
                }
                "Not Medically Necessary (Non-Compliance Non-Medically Necessary)" => {
                    Some(Code3)
                }
                "Not Followed Other (Non-Compliance Other)" => Some(Code4),
                "Emergency Admit to Non-Network Hospital" => {
                    Some(EmergencyAdmitToNonNetworkHospital)
                }
                _ => None,
            }
        }
    }
}
impl Serialize for PolicyComplianceCode {
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
    type Value = PolicyComplianceCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Policy Compliance Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PolicyComplianceCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Policy Compliance Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        PolicyComplianceCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Policy Compliance Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for PolicyComplianceCode {
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