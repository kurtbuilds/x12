use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1350

See docs at <https://www.stedi.com/edi/x12/element/1350>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OxygenTestFindingsCode {
    ///1 - Dependent edema suggesting congestive heart failure
    DependentEdemaSuggestingCongestiveHeartFailure,
    ///2 - "P" Pulmonale on Electrocardiogram (EKG)
    Code2,
    ///3 - Erythrocythemia with a hematocrit greater than 56 percent
    ErythrocythemiaWithAHematocritGreaterThan56Percent,
}
impl OxygenTestFindingsCode {
    pub fn code(&self) -> &str {
        {
            use OxygenTestFindingsCode::*;
            match self {
                DependentEdemaSuggestingCongestiveHeartFailure => "1",
                Code2 => "2",
                ErythrocythemiaWithAHematocritGreaterThan56Percent => "3",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<OxygenTestFindingsCode> {
        use OxygenTestFindingsCode::*;
        match code {
            b"1" => Some(DependentEdemaSuggestingCongestiveHeartFailure),
            b"2" => Some(Code2),
            b"3" => Some(ErythrocythemiaWithAHematocritGreaterThan56Percent),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use OxygenTestFindingsCode::*;
        match self {
            DependentEdemaSuggestingCongestiveHeartFailure => {
                "Dependent edema suggesting congestive heart failure"
            }
            Code2 => "\"P\" Pulmonale on Electrocardiogram (EKG)",
            ErythrocythemiaWithAHematocritGreaterThan56Percent => {
                "Erythrocythemia with a hematocrit greater than 56 percent"
            }
        }
    }
    fn from_description(description: &str) -> Option<OxygenTestFindingsCode> {
        {
            use OxygenTestFindingsCode::*;
            match description {
                "Dependent edema suggesting congestive heart failure" => {
                    Some(DependentEdemaSuggestingCongestiveHeartFailure)
                }
                "\"P\" Pulmonale on Electrocardiogram (EKG)" => Some(Code2),
                "Erythrocythemia with a hematocrit greater than 56 percent" => {
                    Some(ErythrocythemiaWithAHematocritGreaterThan56Percent)
                }
                _ => None,
            }
        }
    }
}
impl Serialize for OxygenTestFindingsCode {
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
    type Value = OxygenTestFindingsCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Oxygen Test Findings Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        OxygenTestFindingsCode::from_description(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Oxygen Test Findings Code: {}", v),
            ))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        OxygenTestFindingsCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Oxygen Test Findings Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for OxygenTestFindingsCode {
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