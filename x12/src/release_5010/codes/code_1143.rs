use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1143

See docs at <https://www.stedi.com/edi/x12-005010/element/1143>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoordinationOfBenefitsCode {
    ///1 - Coordination of Benefits
    CoordinationOfBenefits,
    ///2 - Coordination of Benefits applies to Spouse Only
    CoordinationOfBenefitsAppliesToSpouseOnly,
    ///3 - Coordination of Benefits applies to Spouse and Dependents
    CoordinationOfBenefitsAppliesToSpouseAndDependents,
    ///4 - Coordination of Benefits applies to Dependents Only
    CoordinationOfBenefitsAppliesToDependentsOnly,
    ///5 - Unknown
    Unknown,
    ///6 - No Coordination of Benefits
    NoCoordinationOfBenefits,
    ///7 - Coordination of Benefits Applies to Subscriber Only
    CoordinationOfBenefitsAppliesToSubscriberOnly,
    ///8 - Conflict in Coordination of Benefit
    ConflictInCoordinationOfBenefit,
    ///9 - Coordination of Benefits Applies to Whole Family
    CoordinationOfBenefitsAppliesToWholeFamily,
}
impl CoordinationOfBenefitsCode {
    pub fn code(&self) -> &str {
        {
            use CoordinationOfBenefitsCode::*;
            match self {
                CoordinationOfBenefits => "1",
                CoordinationOfBenefitsAppliesToSpouseOnly => "2",
                CoordinationOfBenefitsAppliesToSpouseAndDependents => "3",
                CoordinationOfBenefitsAppliesToDependentsOnly => "4",
                Unknown => "5",
                NoCoordinationOfBenefits => "6",
                CoordinationOfBenefitsAppliesToSubscriberOnly => "7",
                ConflictInCoordinationOfBenefit => "8",
                CoordinationOfBenefitsAppliesToWholeFamily => "9",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<CoordinationOfBenefitsCode> {
        use CoordinationOfBenefitsCode::*;
        match code {
            b"1" => Some(CoordinationOfBenefits),
            b"2" => Some(CoordinationOfBenefitsAppliesToSpouseOnly),
            b"3" => Some(CoordinationOfBenefitsAppliesToSpouseAndDependents),
            b"4" => Some(CoordinationOfBenefitsAppliesToDependentsOnly),
            b"5" => Some(Unknown),
            b"6" => Some(NoCoordinationOfBenefits),
            b"7" => Some(CoordinationOfBenefitsAppliesToSubscriberOnly),
            b"8" => Some(ConflictInCoordinationOfBenefit),
            b"9" => Some(CoordinationOfBenefitsAppliesToWholeFamily),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use CoordinationOfBenefitsCode::*;
        match self {
            CoordinationOfBenefits => "Coordination of Benefits",
            CoordinationOfBenefitsAppliesToSpouseOnly => {
                "Coordination of Benefits applies to Spouse Only"
            }
            CoordinationOfBenefitsAppliesToSpouseAndDependents => {
                "Coordination of Benefits applies to Spouse and Dependents"
            }
            CoordinationOfBenefitsAppliesToDependentsOnly => {
                "Coordination of Benefits applies to Dependents Only"
            }
            Unknown => "Unknown",
            NoCoordinationOfBenefits => "No Coordination of Benefits",
            CoordinationOfBenefitsAppliesToSubscriberOnly => {
                "Coordination of Benefits Applies to Subscriber Only"
            }
            ConflictInCoordinationOfBenefit => "Conflict in Coordination of Benefit",
            CoordinationOfBenefitsAppliesToWholeFamily => {
                "Coordination of Benefits Applies to Whole Family"
            }
        }
    }
    fn from_description(description: &str) -> Option<CoordinationOfBenefitsCode> {
        {
            use CoordinationOfBenefitsCode::*;
            match description {
                "Coordination of Benefits" => Some(CoordinationOfBenefits),
                "Coordination of Benefits applies to Spouse Only" => {
                    Some(CoordinationOfBenefitsAppliesToSpouseOnly)
                }
                "Coordination of Benefits applies to Spouse and Dependents" => {
                    Some(CoordinationOfBenefitsAppliesToSpouseAndDependents)
                }
                "Coordination of Benefits applies to Dependents Only" => {
                    Some(CoordinationOfBenefitsAppliesToDependentsOnly)
                }
                "Unknown" => Some(Unknown),
                "No Coordination of Benefits" => Some(NoCoordinationOfBenefits),
                "Coordination of Benefits Applies to Subscriber Only" => {
                    Some(CoordinationOfBenefitsAppliesToSubscriberOnly)
                }
                "Conflict in Coordination of Benefit" => {
                    Some(ConflictInCoordinationOfBenefit)
                }
                "Coordination of Benefits Applies to Whole Family" => {
                    Some(CoordinationOfBenefitsAppliesToWholeFamily)
                }
                _ => None,
            }
        }
    }
}
impl Serialize for CoordinationOfBenefitsCode {
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
    type Value = CoordinationOfBenefitsCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Coordination of Benefits Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CoordinationOfBenefitsCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Coordination of Benefits Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CoordinationOfBenefitsCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Coordination of Benefits Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for CoordinationOfBenefitsCode {
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