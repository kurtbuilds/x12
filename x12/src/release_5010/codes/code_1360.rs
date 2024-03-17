use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1360

See docs at <https://www.stedi.com/edi/x12-005010/element/1360>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProviderAgreementCode {
    ///D - Managed Dental Care Program
    ManagedDentalCareProgram,
    ///E - Civilian Health and Medical Program of the Uniformed Services (CHAMPUS) "External" Partnership Agreement
    CodeE,
    ///H - Health Maintenance Organization (HMO) Agreement
    CodeH,
    ///I - Civilian Health and Medical Program of the Uniformed Services (CHAMPUS) "Internal" Partnership Agreement
    CodeI,
    ///N - No Agreement
    NoAgreement,
    ///P - Participation Agreement
    ParticipationAgreement,
    ///Y - Preferred Provider Organization (PPO) Agreement
    CodeY,
}
impl ProviderAgreementCode {
    pub fn code(&self) -> &str {
        {
            use ProviderAgreementCode::*;
            match self {
                ManagedDentalCareProgram => "D",
                CodeE => "E",
                CodeH => "H",
                CodeI => "I",
                NoAgreement => "N",
                ParticipationAgreement => "P",
                CodeY => "Y",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ProviderAgreementCode> {
        use ProviderAgreementCode::*;
        match code {
            b"D" => Some(ManagedDentalCareProgram),
            b"E" => Some(CodeE),
            b"H" => Some(CodeH),
            b"I" => Some(CodeI),
            b"N" => Some(NoAgreement),
            b"P" => Some(ParticipationAgreement),
            b"Y" => Some(CodeY),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ProviderAgreementCode::*;
        match self {
            ManagedDentalCareProgram => "Managed Dental Care Program",
            CodeE => {
                "Civilian Health and Medical Program of the Uniformed Services (CHAMPUS) \"External\" Partnership Agreement"
            }
            CodeH => "Health Maintenance Organization (HMO) Agreement",
            CodeI => {
                "Civilian Health and Medical Program of the Uniformed Services (CHAMPUS) \"Internal\" Partnership Agreement"
            }
            NoAgreement => "No Agreement",
            ParticipationAgreement => "Participation Agreement",
            CodeY => "Preferred Provider Organization (PPO) Agreement",
        }
    }
    fn from_description(description: &str) -> Option<ProviderAgreementCode> {
        {
            use ProviderAgreementCode::*;
            match description {
                "Managed Dental Care Program" => Some(ManagedDentalCareProgram),
                "Civilian Health and Medical Program of the Uniformed Services (CHAMPUS) \"External\" Partnership Agreement" => {
                    Some(CodeE)
                }
                "Health Maintenance Organization (HMO) Agreement" => Some(CodeH),
                "Civilian Health and Medical Program of the Uniformed Services (CHAMPUS) \"Internal\" Partnership Agreement" => {
                    Some(CodeI)
                }
                "No Agreement" => Some(NoAgreement),
                "Participation Agreement" => Some(ParticipationAgreement),
                "Preferred Provider Organization (PPO) Agreement" => Some(CodeY),
                _ => None,
            }
        }
    }
}
impl Serialize for ProviderAgreementCode {
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
    type Value = ProviderAgreementCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Provider Agreement Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ProviderAgreementCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Provider Agreement Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ProviderAgreementCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Provider Agreement Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for ProviderAgreementCode {
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