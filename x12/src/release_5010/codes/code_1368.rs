use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1368

See docs at <https://www.stedi.com/edi/x12/element/1368>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToothStatusCode {
    ///E - To Be Extracted
    ToBeExtracted,
    ///I - Impacted
    Impacted,
    ///M - Missing
    Missing,
    ///X - Extracted
    Extracted,
}
impl ToothStatusCode {
    pub fn code(&self) -> &str {
        {
            use ToothStatusCode::*;
            match self {
                ToBeExtracted => "E",
                Impacted => "I",
                Missing => "M",
                Extracted => "X",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<ToothStatusCode> {
        use ToothStatusCode::*;
        match code {
            b"E" => Some(ToBeExtracted),
            b"I" => Some(Impacted),
            b"M" => Some(Missing),
            b"X" => Some(Extracted),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use ToothStatusCode::*;
        match self {
            ToBeExtracted => "To Be Extracted",
            Impacted => "Impacted",
            Missing => "Missing",
            Extracted => "Extracted",
        }
    }
    fn from_description(description: &str) -> Option<ToothStatusCode> {
        {
            use ToothStatusCode::*;
            match description {
                "To Be Extracted" => Some(ToBeExtracted),
                "Impacted" => Some(Impacted),
                "Missing" => Some(Missing),
                "Extracted" => Some(Extracted),
                _ => None,
            }
        }
    }
}
impl Serialize for ToothStatusCode {
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
    type Value = ToothStatusCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Tooth Status Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ToothStatusCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Tooth Status Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ToothStatusCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!("Invalid Tooth Status Code: {}", std::str::from_utf8(v).unwrap()),
            ))
    }
}
impl<'de> Deserialize<'de> for ToothStatusCode {
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