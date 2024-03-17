use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1527

See docs at <https://www.stedi.com/edi/x12-005010/element/1527>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExceptionCode {
    ///1 - Non-Network Professional Provider in Network Hospital
    NonNetworkProfessionalProviderInNetworkHospital,
    ///2 - Emergency Care
    EmergencyCare,
    ///3 - Services or Specialist not in Network
    ServicesOrSpecialistNotInNetwork,
    ///4 - Out-of-Service Area
    OutOfServiceArea,
    ///5 - State Mandates
    StateMandates,
    ///6 - Other
    Other,
}
impl ExceptionCode {
    pub fn code(&self) -> &str {
        {
            use ExceptionCode::*;
            match self {
                NonNetworkProfessionalProviderInNetworkHospital => "1",
                EmergencyCare => "2",
                ServicesOrSpecialistNotInNetwork => "3",
                OutOfServiceArea => "4",
                StateMandates => "5",
                Other => "6",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ExceptionCode> {
        use ExceptionCode::*;
        match code {
            b"1" => Some(NonNetworkProfessionalProviderInNetworkHospital),
            b"2" => Some(EmergencyCare),
            b"3" => Some(ServicesOrSpecialistNotInNetwork),
            b"4" => Some(OutOfServiceArea),
            b"5" => Some(StateMandates),
            b"6" => Some(Other),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ExceptionCode::*;
        match self {
            NonNetworkProfessionalProviderInNetworkHospital => {
                "Non-Network Professional Provider in Network Hospital"
            }
            EmergencyCare => "Emergency Care",
            ServicesOrSpecialistNotInNetwork => "Services or Specialist not in Network",
            OutOfServiceArea => "Out-of-Service Area",
            StateMandates => "State Mandates",
            Other => "Other",
        }
    }
    fn from_description(description: &str) -> Option<ExceptionCode> {
        {
            use ExceptionCode::*;
            match description {
                "Non-Network Professional Provider in Network Hospital" => {
                    Some(NonNetworkProfessionalProviderInNetworkHospital)
                }
                "Emergency Care" => Some(EmergencyCare),
                "Services or Specialist not in Network" => {
                    Some(ServicesOrSpecialistNotInNetwork)
                }
                "Out-of-Service Area" => Some(OutOfServiceArea),
                "State Mandates" => Some(StateMandates),
                "Other" => Some(Other),
                _ => None,
            }
        }
    }
}
impl Serialize for ExceptionCode {
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
    type Value = ExceptionCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Exception Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ExceptionCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Exception Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ExceptionCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Exception Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for ExceptionCode {
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