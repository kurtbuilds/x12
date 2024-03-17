use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1340

See docs at <https://www.stedi.com/edi/x12-005010/element/1340>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MultipleProcedureCode {
    ///N - Non-Multiple Procedure
    NonMultipleProcedure,
    ///P - Primary
    Primary,
    ///S - Secondary
    Secondary,
}
impl MultipleProcedureCode {
    pub fn code(&self) -> &str {
        {
            use MultipleProcedureCode::*;
            match self {
                NonMultipleProcedure => "N",
                Primary => "P",
                Secondary => "S",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<MultipleProcedureCode> {
        use MultipleProcedureCode::*;
        match code {
            b"N" => Some(NonMultipleProcedure),
            b"P" => Some(Primary),
            b"S" => Some(Secondary),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use MultipleProcedureCode::*;
        match self {
            NonMultipleProcedure => "Non-Multiple Procedure",
            Primary => "Primary",
            Secondary => "Secondary",
        }
    }
    fn from_description(description: &str) -> Option<MultipleProcedureCode> {
        {
            use MultipleProcedureCode::*;
            match description {
                "Non-Multiple Procedure" => Some(NonMultipleProcedure),
                "Primary" => Some(Primary),
                "Secondary" => Some(Secondary),
                _ => None,
            }
        }
    }
}
impl Serialize for MultipleProcedureCode {
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
    type Value = MultipleProcedureCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Multiple Procedure Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        MultipleProcedureCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Multiple Procedure Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        MultipleProcedureCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Multiple Procedure Code: {}", std::str::from_utf8(v)
                    .unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for MultipleProcedureCode {
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