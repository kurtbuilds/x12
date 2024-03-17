use std::fmt;
use serde::{de, Deserialize, ser, Serialize};
/**1333

See docs at <https://www.stedi.com/edi/x12-005010/element/1333>*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecordFormatCode {
    ///C - Content
    Content,
    ///D - Definition
    Definition,
    ///E - Formula
    Formula,
    ///F - Federal
    Federal,
    ///P - Plan
    Plan,
    ///S - State
    State,
}
impl RecordFormatCode {
    pub fn code(&self) -> &str {
        {
            use RecordFormatCode::*;
            match self {
                Content => "C",
                Definition => "D",
                Formula => "E",
                Federal => "F",
                Plan => "P",
                State => "S",
            }
        }
    }
    pub fn from_code(code: &[u8]) -> Option<RecordFormatCode> {
        use RecordFormatCode::*;
        match code {
            b"C" => Some(Content),
            b"D" => Some(Definition),
            b"E" => Some(Formula),
            b"F" => Some(Federal),
            b"P" => Some(Plan),
            b"S" => Some(State),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        use RecordFormatCode::*;
        match self {
            Content => "Content",
            Definition => "Definition",
            Formula => "Formula",
            Federal => "Federal",
            Plan => "Plan",
            State => "State",
        }
    }
    fn from_description(description: &str) -> Option<RecordFormatCode> {
        {
            use RecordFormatCode::*;
            match description {
                "Content" => Some(Content),
                "Definition" => Some(Definition),
                "Formula" => Some(Formula),
                "Federal" => Some(Federal),
                "Plan" => Some(Plan),
                "State" => Some(State),
                _ => None,
            }
        }
    }
}
impl Serialize for RecordFormatCode {
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
    type Value = RecordFormatCode;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Record Format Code")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        RecordFormatCode::from_description(v)
            .ok_or_else(|| E::custom(format!("Invalid Record Format Code: {}", v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        RecordFormatCode::from_code(v)
            .ok_or_else(|| E::custom(
                format!(
                    "Invalid Record Format Code: {}", std::str::from_utf8(v).unwrap()
                ),
            ))
    }
}
impl<'de> Deserialize<'de> for RecordFormatCode {
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